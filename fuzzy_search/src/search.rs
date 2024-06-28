use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
include!(concat!(env!("OUT_DIR"), "./generated_website_data.rs"));

const LIMIT: usize = 12;

static WEBSITE_DATA_CHUNK: Lazy<Mutex<Vec<&WebsiteData>>> = Lazy::new(|| Mutex::new(Vec::new()));
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    id: u64,
    title: String,
    description: String,
    thumbnail: String,
    url: String,
    score: u16,
    tags: Vec<i32>,
}

#[must_use]
#[wasm_bindgen]
pub fn init(worker_count: usize, worker_idx: usize) {
    let new_slice = WEBSITE_DATA
        .chunks_exact(WEBSITE_DATA.len() / worker_count)
        .nth(worker_idx)
        .expect("couldn't get a website data slice");
    let mut chunk = WEBSITE_DATA_CHUNK.lock().unwrap();
    chunk.extend(new_slice);
}

#[must_use]
#[wasm_bindgen]
pub fn search(query: String, filter_by_tags: Vec<i32>) -> Result<JsValue, JsValue> {
    let mut config = nucleo::Config::DEFAULT;
    config.set_match_paths();
    let mut nucleo = nucleo::Matcher::new(config);
    let mut matches: Vec<(&WebsiteData, u16)> = Vec::with_capacity(2048);
    let utf_query = Utf32Str::Ascii(query.as_bytes());
    let chunk = WEBSITE_DATA_CHUNK.lock().unwrap();
    if query.is_empty() {
        if !filter_by_tags.is_empty() {
            let mut matches: Vec<(&WebsiteData, u16)> = Vec::with_capacity(LIMIT);
            for website in chunk.iter().filter(|website_data| {
                filter_by_tags
                    .iter()
                    .all(|item| website_data.tags.contains(item))
            }) {
                matches.push((website, 1));
                if matches.len() == LIMIT {
                    let results: Vec<SearchResult> = matches
                        .into_iter()
                        .take(LIMIT)
                        .map(|(website, score)| SearchResult {
                            title: website.title.to_string(),
                            url: website.url.to_string(),
                            description: website.description.to_string(),
                            thumbnail: website.thumbnail.to_string(),
                            id: website.id,
                            score,
                            tags: website
                                .tags
                                .iter()
                                .filter(|&&index| index != -1)
                                .map(|&index| index)
                                .collect(),
                        })
                        .collect();
                    return Ok(serde_wasm_bindgen::to_value(&results)?);
                }
            }
        }
    }
    if filter_by_tags.is_empty() {
        for website in chunk.iter() {
            let mut website_score = 0;
            if let Some(score) =
                nucleo.fuzzy_match(Utf32Str::Ascii(website.title.as_bytes()), utf_query)
            {
                website_score += score;
            }
            if let Some(score) =
                nucleo.fuzzy_match(Utf32Str::Ascii(website.content.as_bytes()), utf_query)
            {
                website_score += score;
            }
            matches.push((website, website_score));
        }
    } else {
        for website in chunk.iter().filter(|website_data| {
            filter_by_tags
                .iter()
                .all(|item| website_data.tags.contains(item))
        }) {
            let mut website_score = 0;
            if let Some(score) =
                nucleo.fuzzy_match(Utf32Str::Ascii(website.title.as_bytes()), utf_query)
            {
                website_score += score;
            }
            if let Some(score) =
                nucleo.fuzzy_match(Utf32Str::Ascii(website.content.as_bytes()), utf_query)
            {
                website_score += score;
            }
            matches.push((website, website_score));
        }
    }
    matches.sort_by(|a, b| b.1.cmp(&a.1));
    let results: Vec<SearchResult> = matches
        .into_iter()
        .take(LIMIT)
        .map(|(website, score)| SearchResult {
            title: website.title.to_string(),
            url: website.url.to_string(),
            description: website.description.to_string(),
            thumbnail: website.thumbnail.to_string(),
            id: website.id,
            score,
            tags: website
                .tags
                .iter()
                .filter(|&&index| index != -1)
                .map(|&index| index)
                .collect(),
        })
        .collect();
    Ok(serde_wasm_bindgen::to_value(&results)?)
}

#[must_use]
#[wasm_bindgen]
pub fn substring_search(query: String, filter_by_tags: Vec<i32>) -> Result<JsValue, JsValue> {
    let mut matches: Vec<(&WebsiteData, u16)> = Vec::with_capacity(2048);
    let chunk = WEBSITE_DATA_CHUNK.lock().unwrap();
    if query.is_empty() {
        if !filter_by_tags.is_empty() {
            let mut matches: Vec<(&WebsiteData, u16)> = Vec::with_capacity(LIMIT);
            for website in chunk.iter().filter(|website_data| {
                filter_by_tags
                    .iter()
                    .all(|item| website_data.tags.contains(item))
            }) {
                matches.push((website, 1));
                if matches.len() == LIMIT {
                    let results: Vec<SearchResult> = matches
                        .into_iter()
                        .take(LIMIT)
                        .map(|(website, score)| SearchResult {
                            title: website.title.to_string(),
                            url: website.url.to_string(),
                            description: website.description.to_string(),
                            thumbnail: website.thumbnail.to_string(),
                            id: website.id,
                            score,
                            tags: website
                                .tags
                                .iter()
                                .filter(|&&index| index != -1)
                                .map(|&index| index)
                                .collect(),
                        })
                        .collect();
                    return Ok(serde_wasm_bindgen::to_value(&results)?);
                }
            }
        }
    }
    let words: Vec<&str> = query.split(" ").collect();
    if filter_by_tags.is_empty() {
        for website in chunk.iter() {
            let mut website_score = 0;
            for word in &words {
                if website.title.contains(word) {
                    website_score += 1;
                }
                if website.content.contains(word) {
                    website_score += 1;
                }
            }
            matches.push((website, website_score));
        }
    } else {
        for website in chunk.iter().filter(|website_data| {
            filter_by_tags
                .iter()
                .all(|item| website_data.tags.contains(item))
        }) {
            let mut website_score = 0;
            for word in &words {
                if website.title.contains(word) {
                    website_score += 1;
                }
                if website.content.contains(word) {
                    website_score += 1;
                }
            }
            matches.push((website, website_score));
        }
    }
    matches.sort_by(|a, b| b.1.cmp(&a.1));
    let results: Vec<SearchResult> = matches
        .into_iter()
        .take(LIMIT)
        .map(|(website, score)| SearchResult {
            title: website.title.to_string(),
            url: website.url.to_string(),
            description: website.description.to_string(),
            thumbnail: website.thumbnail.to_string(),
            id: website.id,
            score,
            tags: website
                .tags
                .iter()
                .filter(|&&index| index != -1)
                .map(|&index| index)
                .collect(),
        })
        .collect();
    Ok(serde_wasm_bindgen::to_value(&results)?)
}

mod tests {
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    #[wasm_bindgen_test]
    fn pass() {
        substring_search("reverse".to_owned(), [1].to_vec()).unwrap();
        // println!("{:?}",result.as_string())
    }
}
