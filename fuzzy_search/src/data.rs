use lazy_static::lazy_static;
use nucleo::Utf32Str;
use serde::{Deserialize, Serialize};

const WEBSITE_DATA_JSON: &str = include_str!("../../scraper/websites.json");

#[derive(Serialize, Deserialize)]
pub struct WebsiteData {
    id: u64,
    title: String,
    content: String,
    url: String,
    tags: Vec<usize>
}

    // pub static WEBSITE_DATA: Vec<WebsiteData> = serde_json::from_str(&WEBSITE_DATA_JSON).unwrap();
    pub const WEBSITE_DATA_STRING:&str = &serde_json::from_str::<Vec<WebsiteData>>(&WEBSITE_DATA_JSON).unwrap().iter().map(|data| data.content.clone()).collect::<Vec<String>>().join(", ");
    pub const WEBSITE_DATA_HAYSTACK:Utf32Str<'static> = Utf32Str::<'static>::Ascii(WEBSITE_DATA_STRING.as_bytes());