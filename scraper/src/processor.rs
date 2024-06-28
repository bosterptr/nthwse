use super::{extractor::ExtractedWebsite, fetcher::fetch_website};
use crate::{
    config::CONFIG,
    globals::{DOWNLOADED_WEBSITES, WEBSITES_DATA, WEBSITES_FULL_DATA},
    loader::load_website_from_disk,
};
use regex::escape;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
};
use tracing::{debug, error, info};
use std::io::Cursor;
use base64::{engine::general_purpose, Engine as _};
use image::codecs::jpeg::JpegEncoder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlIDPair {
    pub id: u64,
    pub url: String,
}

async fn save_to_file(filename: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn save_failed_download(website: &ExtractedWebsite) -> Result<(), std::io::Error> {
    let mut failed_downloads = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .open(CONFIG.failed_downloads_path.clone())
        .await?;
    failed_downloads
        .write_all(format!("{}\n", website.url).as_bytes())
        .await
        .expect("Failed to write to failed_downloads file");
    let _ = failed_downloads.flush();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebsiteData {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub url: String,
    pub description: String,
    pub thumbnail: String,
    pub tags: Vec<usize>,
}

fn get_title(content: &str, extracted_website: &ExtractedWebsite) -> String {
    let document: Html = Html::parse_document(&content);
    let mut title = extracted_website.title.clone();
    if title != "" {
        return title;
    }
    if let Some(element) = document.select(&Selector::parse("title").unwrap()).next() {
        if let Some(content) = element.value().attr("content") {
            title = content.to_owned();
        }
    }
    title.to_owned()
}

fn get_description(content: &str) -> String {
    let document: Html = Html::parse_document(&content);
    let mut description = "";
    let selectors = [
        Selector::parse(r#"meta[property="og:description"]"#).unwrap(),
        Selector::parse(r#"meta[name="description"]"#).unwrap(),
        Selector::parse(r#"meta[name="twitter:description"]"#).unwrap(),
    ];
    for selector in &selectors {
        if let Some(element) = document.select(selector).next() {
            if let Some(content) = element.value().attr("content") {
                description = content;
            }
        }
    }
    description.to_owned()
}

fn filter_and_format_elements(content: &str) -> String {
    let mut document: Html = Html::parse_document(&content);
    let code_selector = Selector::parse("code").unwrap();
    let selectors = ["span", "p", "h1", "h2", "h3", "h4", "h5", "h6"];
    let mut formatted_text = String::new();
    let node_ids: Vec<_> = document.select(&code_selector).map(|x| x.id()).collect();
    for id in node_ids {
        document.tree.get_mut(id).unwrap().detach();
    }
    for selector in &selectors {
        let element_selector = Selector::parse(selector).unwrap();
        for element in document.select(&element_selector) {
            let text: String = element.text().collect();
            formatted_text.push_str(&text.trim());
            formatted_text.push(' ');
        }
    }
    formatted_text
        .to_string()
        .replace("\n", "")
        .replace("\\n", "")
        .replace("{", "")
        .replace("}", "")
        .replace("<", "")
        .replace("(", "")
        .replace(")", "")
        .replace(".", "")
        .replace("\\", "")
        .replace("\"", "")
        .replace("|", "")
        .replace("[", "")
        .replace("]", "")
        .replace("'", "")
        .replace("·", "")
        .replace("\\\"", "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .to_string()
}
pub async fn process_website(website: &ExtractedWebsite, client: &fantoccini::Client) {
    info!("processing {}", website.clone().url);
    let loaded_website = match CONFIG.update {
        true => fetch_website(website.clone(), &client).await,
        false => load_website_from_disk(website.clone()).await,
    };
    match loaded_website {
        Ok(content) => {
            info!("fetched {}", website.clone().url);
            let title: &str = &get_title(&content, website);
            let description: &str = &get_description(&content);
            let formatted_content: &str = &filter_and_format_elements(&content);
            let unique_filename = format!("{}/{}.html", CONFIG.nthw_unique_words_path, website.id);
            let filename = format!("{}/{}.html", CONFIG.nthw_raw_websites_path, website.id);
            info!("filtered {}", website.clone().url);
            let unique_words: HashSet<String> = formatted_content
                .split_whitespace()
                .map(|word| word.to_lowercase())
                .collect();
            let unique_content: String =
                escape(&unique_words.iter().cloned().collect::<Vec<_>>().join(" "));
            info!("unique_content {}", website.clone().url);
            DOWNLOADED_WEBSITES.lock().unwrap().push(UrlIDPair {
                id: website.id,
                url: website.url.clone().replace("\\\"", "").replace("\"", ""),
            });
            info!("saving {}", website.clone().url);
            match save_to_file(&unique_filename, &unique_content).await {
                Ok(_) => debug!("Saved {} content to {}", title, unique_filename),
                Err(e) => {
                    save_failed_download(website).await.unwrap();
                    error!(
                        "Failed to save {} content: {} {}",
                        title, e, unique_filename
                    );
                }
            }
            info!("saving {}", website.clone().url);
            match save_to_file(&filename, &content).await {
                Ok(_) => debug!("Saved {} content to {}", title, filename),
                Err(e) => {
                    save_failed_download(website).await.unwrap();
                    error!("Failed to save {} content: {} {}", title, e, filename);
                }
            }
            let img_filename =
                format!("{}/{}.png", CONFIG.generate_resized_screenshots, website.id);
            let input_img_filename = format!("screenshots/{}.png", website.id);
            if CONFIG.generate_resized_screenshots {
                let img = image::open(input_img_filename.clone()).unwrap();
                let scaled = img.resize(455, 250, image::imageops::FilterType::Gaussian);
                scaled.save(img_filename).unwrap();
            }
            let img: image::DynamicImage = image::open(input_img_filename).unwrap();
            let thumbnail = img.resize(50, 27, image::imageops::FilterType::Gaussian);
            let mut buffer = Vec::new();
            {
                let mut cursor = Cursor::new(&mut buffer);
                let mut encoder = JpegEncoder::new_with_quality(&mut cursor, 80);
                encoder.encode_image(&thumbnail).expect("Failed to write image to buffer");
            }
            let base64_thumbnail = general_purpose::STANDARD.encode(&buffer).replace("/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAbADIDAREAAhEBAxEB/9sAQwAUDg8SDw0UEhASFxUUGB4yIR4cHB49LC4kMklATEtHQEZFUFpzYlBVbVZFRmSIZW13e4GCgU5gjZeMfZZzfoF8/9sAQwEVFxceGh47ISE7fFNGU3x8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/", "");
            if !WEBSITES_FULL_DATA.lock().unwrap().iter().any(|website_data| website_data.id == website.id) {
                WEBSITES_FULL_DATA.lock().unwrap().push(WebsiteData {
                    id: website.id,
                    url: website.url.clone().replace("\\\"", "").replace("\"", ""),
                    content: formatted_content.to_owned(),
                    thumbnail: base64_thumbnail.clone(),
                    title: title.to_owned(),
                    description: description.to_owned(),
                    tags: website.tags.clone(),
                });
                WEBSITES_DATA.lock().unwrap().push(WebsiteData {
                    id: website.id,
                    url: website.url.clone().replace("\\\"", "").replace("\"", ""),
                    content: unique_content,
                    thumbnail: base64_thumbnail,
                    title: title.to_owned(),
                    description: description.to_owned(),
                    tags: website.tags.clone(),
                });
            }
            info!("end {}", website.clone().url);
        }
        Err(_) => {
            save_failed_download(website).await.unwrap();
        }
    }
}

pub async fn process_website_from_disk(website: &ExtractedWebsite) {
    info!("processing {} {}", website.clone().id, website.clone().url);
    match load_website_from_disk(website.clone()).await {
        Ok(content) => {
            let title: &str = &get_title(&content, website);
            let description: &str = &get_description(&content);
            let formatted_content: &str = &filter_and_format_elements(&content);
            let unique_filename = format!("{}/{}.html", CONFIG.nthw_unique_words_path, website.id);
            let unique_words: HashSet<String> = formatted_content
                .split_whitespace()
                .map(|word| word.to_lowercase())
                .collect();
            let unique_content: String = unique_words
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
                .escape_default()
                .to_string();
            debug!("fetched {}", unique_content.len());
            match save_to_file(&unique_filename, &unique_content).await {
                Ok(_) => debug!("Saved {} content to {}", title, unique_filename),
                Err(e) => {
                    save_failed_download(website).await.unwrap();
                    error!(
                        "Failed to save {} content: {} {}",
                        title, e, unique_filename
                    );
                }
            }
            let img_filename = format!("{}/{}.png", CONFIG.resized_screenshots_path, website.id);
            let input_img_filename = format!("./screenshots/{}.png", website.id);
            if CONFIG.generate_resized_screenshots {
                let img = image::open(input_img_filename.clone()).unwrap();
                let scaled = img.resize(455, 250, image::imageops::FilterType::Gaussian);
                scaled.save(img_filename).unwrap();
            }
            if !DOWNLOADED_WEBSITES.lock().unwrap().iter().any(|pair| pair.id == website.id) {
                DOWNLOADED_WEBSITES.lock().unwrap().push(UrlIDPair {
                    id: website.id,
                    url: website.url.clone().replace("\\\"", "").replace("\"", ""),
                });
            }
            let img: image::DynamicImage = image::open(input_img_filename).unwrap();
            let thumbnail = img.resize(50, 27, image::imageops::FilterType::Gaussian);
            let mut buffer = Vec::new();
            {
                let mut cursor = Cursor::new(&mut buffer);
                let mut encoder = JpegEncoder::new_with_quality(&mut cursor, 40);
                encoder.encode_image(&thumbnail).expect("Failed to write image to buffer");
            }
            let base64_thumbnail = general_purpose::STANDARD.encode(&buffer).replace("/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAbADIDAREAAhEBAxEB/9sAQwAUDg8SDw0UEhASFxUUGB4yIR4cHB49LC4kMklATEtHQEZFUFpzYlBVbVZFRmSIZW13e4GCgU5gjZeMfZZzfoF8/9sAQwEVFxceGh47ISE7fFNGU3x8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/", "");
            if !WEBSITES_FULL_DATA.lock().unwrap().iter().any(|website_data| website_data.id == website.id) {
                WEBSITES_FULL_DATA.lock().unwrap().push(WebsiteData {
                    id: website.id,
                    url: website.url.clone().replace("\\\"", "").replace("\"", ""),
                    content: formatted_content.to_owned(),
                    thumbnail: base64_thumbnail.clone(),
                    title: title.to_owned(),
                    description: description.to_owned(),
                    tags: website.tags.clone(),
                });
                WEBSITES_DATA.lock().unwrap().push(WebsiteData {
                    id: website.id,
                    url: website.url.clone().replace("\\\"", "").replace("\"", ""),
                    content: unique_content,
                    thumbnail: base64_thumbnail.clone(),
                    title: title.to_owned(),
                    description: description.to_owned(),
                    tags: website.tags.clone(),
                });
            }
        }
        Err(_) => {
            save_failed_download(website).await.unwrap();
        }
    }
}
