#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
use bincode::{DefaultOptions, Options};
use core::fmt;
use once_cell::sync::Lazy;
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize as SerSerialize, SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashSet;

static EMBEDDINGS: Lazy<Vec<WebsiteEmbeddingsWithData>> = Lazy::new(|| {
    DefaultOptions::new()
        .with_varint_encoding()
        .deserialize::<Vec<WebsiteEmbeddingsWithData>>(include_bytes!(concat!(
            env!("OUT_DIR"),
            "./embeddings.bin"
        )))
        .unwrap()
});
const CHUNK_WORD_COUNT: usize = 100;

#[derive(Serialize, Deserialize)]
pub struct AISearchResult {
    id: u64,
    title: String,
    description: String,
    // content: String,
    thumbnail: String,
    url: String,
    score: f32,
    tags: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Embedding([f32; 768]);
impl Embedding {
    fn from_vec(vec: Vec<f32>) -> Option<Self> {
        if vec.len() == 768 {
            let array: [f32; 768] = match vec.try_into() {
                Ok(arr) => arr,
                Err(_) => return None,
            };
            Some(Embedding(array))
        } else {
            None
        }
    }
}

impl SerSerialize for Embedding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for element in &self.0 {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Embedding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EmbeddingVisitor;

        impl<'de> Visitor<'de> for EmbeddingVisitor {
            type Value = Embedding;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of 768 floats")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Embedding, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut array = [0.0f32; 768];
                for i in 0..768 {
                    array[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                Ok(Embedding(array))
            }
        }

        deserializer.deserialize_seq(EmbeddingVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WebsiteEmbeddingsWithData {
    id: u64,
    embeddings: Vec<Embedding>,
    title: String,
    content: String,
    thumbnail: String,
    description: String,
    url: String,
    tags: Vec<i32>,
}

const LIMIT: usize = 12;
fn magnitude(arr: &Embedding) -> f32 {
    let mut sum_of_squares = 0.0;
    for &val in arr.0.iter() {
        sum_of_squares += val * val;
    }
    sum_of_squares.sqrt()
}
fn dot(arr1: &Embedding, arr2: &Embedding) -> f32 {
    let mut result = 0.0;
    for i in 0..arr1.0.len() {
        result += arr1.0[i] * arr2.0[i];
    }
    result
}

fn cos_sim(arr1: &Embedding, arr2: &Embedding) -> f32 {
    let dot_product = dot(arr1, arr2);
    let magnitude_a = magnitude(arr1);
    let magnitude_b = magnitude(arr2);
    dot_product / (magnitude_a * magnitude_b)
}

fn get_word_chunks(content: &str, chunk_size:usize) -> Vec<Vec<&str>>  {
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut chunks: Vec<Vec<&str>>  = Vec::new();
    for chunk in words.chunks(chunk_size) {
        chunks.push(chunk.to_vec());
    }
    chunks
}

#[must_use]
#[wasm_bindgen]
pub fn search(
    query_embeddings_vec: Vec<f32>,
    filter_by_tags: Vec<i32>,
) -> Result<JsValue, JsValue> {
    let query_embeddings = Embedding::from_vec(query_embeddings_vec).unwrap();
    let mut matches: Vec<(&WebsiteEmbeddingsWithData, f32, usize)> = Vec::with_capacity(4096);
    let mut seen_ids = HashSet::new();
    let chunk = &EMBEDDINGS;
    if filter_by_tags.is_empty() {
        for website in chunk.iter() {
            if let Some((max_similarity, max_index)) = website
                .embeddings
                .iter()
                .enumerate()
                .map(|(index, website_embedding)| {
                    (cos_sim(website_embedding, &query_embeddings), index)
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            {
                if seen_ids.insert(website.id) {
                    matches.push((website, max_similarity, max_index));
                }
            }
        }
    } else {
        for website in chunk.iter().filter(|website| {
            filter_by_tags
                .iter()
                .any(|item| website.tags.contains(item))
        }) {

            if let Some((max_similarity, max_index)) = website
                .embeddings
                .iter()
                .enumerate()
                .map(|(index, website_embedding)| {
                    (cos_sim(website_embedding, &query_embeddings), index)
                })
                .max_by(|a, b| {
                    a.0.partial_cmp(&b.0).unwrap()})
            {
                if seen_ids.insert(website.id) {
                    matches.push((website, max_similarity, max_index));
                }
            }
        }
    }
    matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let results: Vec<AISearchResult> = matches
        .into_iter()
        .take(LIMIT)
        .map(|(website, score, max_index)| {
            // let content_chunks = get_word_chunks(&website.content, CHUNK_WORD_COUNT);
            AISearchResult {
            title: website.title.to_string(),
            url: website.url.to_string(),
            description: website.description.to_string(),
            thumbnail: website.thumbnail.to_string(),
            // content: content_chunks[max_index].clone().join(" "),
            id: website.id,
            score,
            tags: website
                .tags
                .iter()
                .filter(|&&index| index != -1)
                .map(|&index| index)
                .collect(),
        }})
        .collect();
    Ok(serde_wasm_bindgen::to_value(&results)?)
}

mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    #[wasm_bindgen_test]
    fn pass() {
        let result = search([0.5;768].to_vec(), [1].to_vec()).unwrap();
        println!("{:?}",result);
    }
}
