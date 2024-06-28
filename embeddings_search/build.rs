use bincode::{DefaultOptions, Options};
use core::fmt;
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize as SerSerialize, SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
struct Embedding([f32; 768]);

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

#[derive(Serialize, Deserialize, Debug)]
struct WebsiteEmbeddings {
    id: u64,
    embeddings: Vec<Embedding>,
}

#[derive(Serialize, Deserialize, Clone)]
struct WebsiteData {
    id: u64,
    title: String,
    description: String,
    content: String,
    thumbnail: String,
    url: String,
    tags: Vec<i32>,
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

fn merge_vecs(
    vec_a: Vec<WebsiteData>,
    vec_b: Vec<WebsiteEmbeddings>,
) -> Vec<WebsiteEmbeddingsWithData> {
    let mut result = Vec::new();
    for a in &vec_a {
        for b in &vec_b {
            if a.id == b.id {
                b.embeddings
                    .iter()
                    .for_each(|embedding| assert_eq!(embedding.0.len(), 768));
                result.push(WebsiteEmbeddingsWithData {
                    id: b.id,
                    embeddings: b.embeddings.clone(),
                    description: a.description.clone(),
                    content: a.content.clone(),
                    thumbnail: a.thumbnail.clone(),
                    tags: a.tags.clone(),
                    title: a.title.clone(),
                    url: a.url.clone(),
                });
            }
        }
    }
    result
}

fn process_websites_json() {
    let website_embeddings_json = include_str!("../embedding_extractor/website_embeddings.json");
    let website_embeddings: Vec<WebsiteEmbeddings> =
        serde_json::from_str(website_embeddings_json).unwrap();
    let website_data_json = include_str!("../scraper/websites.json");
    let website_data: Vec<WebsiteData> = serde_json::from_str(website_data_json).unwrap();
    let mut seen_ids = HashSet::new();
    let mut unique_data = Vec::new();
    for item in website_data {
        if seen_ids.insert(item.id) {
            unique_data.push(item);
        }
    }
    let website_embeddings_with_data = merge_vecs(unique_data, website_embeddings);

    let bytes = DefaultOptions::new()
        .with_varint_encoding()
        .serialize(&website_embeddings_with_data)
        .unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embeddings.bin");
    fs::write(dest_path, bytes).unwrap();

    //     let websites_embeddings: Vec<WebsiteEmbeddings> = serde_json::from_str(website_embeddings_json).unwrap();
    //     let mut generated_code = String::new();
    //     generated_code.push_str(
    //         r#"
    // use serde::ser::{Serialize as SerSerialize, Serializer, SerializeSeq};
    // use serde::de::{self, Deserializer, Visitor, SeqAccess};
    // use serde::{Deserialize, Serialize};
    // use serde_json;
    // use core::fmt;
    // use std::fs;
    // use std::path::Path;

    // #[derive(Debug, PartialEq)]
    // struct Embedding([f32; 768]);

    // impl SerSerialize for Embedding {
    //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //     where
    //         S: Serializer,
    //     {
    //         let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
    //         for element in &self.0 {
    //             seq.serialize_element(element)?;
    //         }
    //         seq.end()
    //     }
    // }

    // impl<'de> Deserialize<'de> for Embedding {
    //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //     where
    //         D: Deserializer<'de>,
    //     {
    //         struct EmbeddingVisitor;

    //         impl<'de> Visitor<'de> for EmbeddingVisitor {
    //             type Value = Embedding;

    //             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    //                 formatter.write_str("an array of 768 floats")
    //             }

    //             fn visit_seq<V>(self, mut seq: V) -> Result<Embedding, V::Error>
    //             where
    //                 V: SeqAccess<'de>,
    //             {
    //                 let mut array = [0.0f32; 768];
    //                 for i in 0..768 {
    //                     array[i] = seq.next_element()?
    //                         .ok_or_else(|| de::Error::invalid_length(i, &self))?;
    //                 }
    //                 Ok(Embedding(array))
    //             }
    //         }

    //         deserializer.deserialize_seq(EmbeddingVisitor)
    //     }
    // }

    // #[derive(Serialize, Deserialize, Debug)]
    // struct WebsiteEmbeddings {
    //     id: u64,
    //     embeddings: Vec<Embedding>,
    // }

    // const WEBSITES_EMBEDDINGS: [WebsiteEmbeddings; "#);
    //     generated_code.push_str(&websites_embeddings.len().to_string());
    //     generated_code.push_str("] = [");
    //     let embeddings_len = websites_embeddings.len();
    //     for data in websites_embeddings {
    //         generated_code.push_str(&format!(
    //             "WebsiteEmbeddings {{ id: {}, embeddings: {:?}}},",
    //             data.id,
    //             data.embeddings
    //         ));
    //     }
    //     if embeddings_len != 0 {
    //         generated_code.remove(generated_code.len() - 1);
    //     }
    //     generated_code.push_str("];");
    //     let out_dir = std::env::var("OUT_DIR").unwrap();
    //     let dest_path = Path::new(&out_dir).join("generated_websites_embeddings.rs");
    //     fs::write(dest_path, generated_code).unwrap();
}

fn main() {
    process_websites_json();
    // process_tags_json();
}
