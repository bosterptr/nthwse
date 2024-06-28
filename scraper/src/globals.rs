use crate::processor::{UrlIDPair, WebsiteData};
use std::sync::Mutex;

pub static TAKEN_PORTS: Mutex<Vec<u16>> = Mutex::new(Vec::new());
pub static DOWNLOADED_WEBSITES: Mutex<Vec<UrlIDPair>> = Mutex::new(Vec::new());
pub static FAILED_DOWNLOADS_WEBSITES: Mutex<Vec<UrlIDPair>> = Mutex::new(Vec::new());
pub static WEBSITES_DATA: Mutex<Vec<WebsiteData>> = Mutex::new(Vec::new());
pub static WEBSITES_FULL_DATA: Mutex<Vec<WebsiteData>> = Mutex::new(Vec::new());
