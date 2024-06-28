use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub base_driver_port: u16,
    pub downloaded_websites_path: String,
    pub failed_downloads_path: String,
    pub generate_resized_screenshots: bool,
    pub resized_screenshots_path: String,
    pub nthw_frontend_json_path: String,
    pub nthw_frontend_tags_path: String,
    pub nthw_git_path: String,
    pub nthw_raw_websites_path: String,
    pub nthw_unique_words_path: String,
    pub update: bool,
    pub nthw_frontend_full_json_path: String,
    pub worker_count: usize,
}

impl Config {
    pub fn new() -> Result<Config, anyhow::Error> {
        Ok(Config {
            base_driver_port: env::var("NTHW_BASE_DRIVER_PORT")?.parse::<u16>()?,
            downloaded_websites_path: env::var("NTHW_DOWNLOADED_WEBSITES_PATH")?,
            failed_downloads_path: env::var("NTHW_FAILED_DOWNLOADS_PATH")?,
            generate_resized_screenshots: env::var("NTHW_GENERATE_RESIZED_SCREENSHOTS")?
                .parse::<bool>()?,
            resized_screenshots_path: env::var("NTHW_RESIZED_SCREENSHOTS_PATH")?,
            nthw_frontend_json_path: env::var("NTHW_FRONTEND_JSON_PATH")?,
            nthw_frontend_tags_path: env::var("NTHW_FRONTEND_TAGS_PATH")?,
            nthw_git_path: env::var("NTHW_NTHW_GIT_PATH")?,
            nthw_raw_websites_path: env::var("NTHW_RAW_WEBSITES_PATH")?,
            nthw_unique_words_path: env::var("NTHW_UNIQUE_WORDS_PATH")?,
            update: env::var("NTHW_UPDATE")?.parse::<bool>()?,
            nthw_frontend_full_json_path: env::var("NTHW_FRONTEND_FULL_JSON_PATH")?,
            worker_count: env::var("NTHW_WORKER_COUNT")?.parse::<usize>()?,
        })
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("couldn't read config"));
