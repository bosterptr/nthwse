use dotenv::dotenv;
use futures::future::join_all;
use nthw::config::{Config, CONFIG};
use nthw::extractor::extract_links;
use nthw::fs::save_to_json;
use nthw::globals::{
    DOWNLOADED_WEBSITES, FAILED_DOWNLOADS_WEBSITES, WEBSITES_DATA, WEBSITES_FULL_DATA,
};
use nthw::processor::process_website_from_disk;
use nthw::scraper_pool_manager::create_pool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::info;
// use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WebsiteData {
    id: u64,
    title: String,
    content: String,
    url: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // console_subscriber::init();
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        // .with_file(true)
        // .with_line_number(false)
        .with_thread_ids(true)
        .with_target(false)
        .finish()
        .init();
    let config = Config::new().expect("coundn't load config");
    let pool = create_pool(CONFIG.base_driver_port, CONFIG.worker_count);
    let downloaded_websites = DOWNLOADED_WEBSITES.lock().unwrap().clone();
    let extracted_websites = extract_links(&config.nthw_git_path, downloaded_websites.to_vec())
        .await
        .expect("couldn't extract links from the repo");
    let mut task_handles = Vec::new();
    let semaphore = Arc::new(Semaphore::new(CONFIG.worker_count));
    if CONFIG.update {
        for website in extracted_websites {
            let permit = Arc::clone(&semaphore).acquire_owned().await;
            info!("retrieving scraper from the pool");
            let pool_clone = pool.clone();
            let task_handle = tokio::spawn(async move {
                let _permit = permit;
                let scraper = pool_clone
                    .get()
                    .await
                    .expect("coundn't retrieve a scraper from a pool");
                scraper.process_website(&website).await
            });
            task_handles.push(task_handle);
        }
        join_all(task_handles).await;
    } else {
        for website in extracted_websites {
            let permit = Arc::clone(&semaphore).acquire_owned().await;
            let task_handle = tokio::spawn(async move {
                let _permit = permit;
                process_website_from_disk(&website).await
            });
            task_handles.push(task_handle);
        }
        join_all(task_handles).await;
    }
    save_to_json(
        &CONFIG.nthw_frontend_json_path,
        WEBSITES_DATA.lock().unwrap().to_vec(),
    )
    .await
    .unwrap();
    save_to_json(
        &CONFIG.nthw_frontend_full_json_path,
        WEBSITES_FULL_DATA.lock().unwrap().to_vec(),
    )
    .await
    .unwrap();
    save_to_json(
        &CONFIG.downloaded_websites_path,
        DOWNLOADED_WEBSITES.lock().unwrap().to_vec(),
    )
    .await
    .unwrap();
    save_to_json(
        &CONFIG.failed_downloads_path,
        FAILED_DOWNLOADS_WEBSITES.lock().unwrap().to_vec(),
    )
    .await
    .unwrap();
}
