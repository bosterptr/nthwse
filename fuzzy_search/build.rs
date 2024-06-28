use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

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

pub const WEBSITE_DATA: [&str; 1] = ["2"];
fn process_websites_json() {
    let website_data_json = include_str!("../scraper/websites.json");

    let website_data: Vec<WebsiteData> = serde_json::from_str(website_data_json).unwrap();

    let mut generated_code = String::new();
    generated_code.push_str(
        "use serde::{Deserialize, Serialize};
use nucleo::Utf32Str;

#[derive(Serialize, Deserialize)]
struct WebsiteData<'a> {
    id: u64,
    title: &'a str,
    description: &'a str,
    content: &'a str,
    thumbnail: &'a str,
    url: &'a str,
    tags: [i32;5]
}
const WEBSITE_DATA: [WebsiteData; ",
    );
    generated_code.push_str(&website_data.len().to_string());
    generated_code.push_str("] = [");

    for data in website_data.clone() {
        let mut tags: [i32; 5] = [-1; 5];
        for (i, &value) in data.tags.iter().enumerate() {
            tags[i] = value as i32;
        }
        generated_code.push_str(&format!(
            "WebsiteData {{ id: {}, title: \"{}\", description: \"{}\", content: \"{}\", thumbnail: \"{}\", url: \"{}\", tags: {:?}  }},",
            data.id,
            data.title.escape_default(),
            data.description.escape_default(),
            data.content.escape_default(),
            data.thumbnail.escape_default(),
            data.url,
            tags
        ));
    }
    if website_data.len() != 0 {
        generated_code.remove(generated_code.len() - 1);
    }
    generated_code.push_str("];");
    // generated_code.push_str("\n");
    // generated_code.push_str(r#"pub const WEBSITE_DATAUTF32STR: Utf32Str =Utf32Str::Ascii(b"#);
    // for data in website_data.clone() {
    //     generated_code.push_str(&format!("{:#?},", data.title.to_ascii_lowercase()));
    // }
    // if website_data.len() != 0 {
    //     generated_code.remove(generated_code.len() - 1);
    // }
    // generated_code.push_str(r#");"#);
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_website_data.rs");
    fs::write(dest_path, generated_code).unwrap();

    //     let website_data_json = include_str!("../scraper/websites.json");

    //     let website_data: Vec<WebsiteData> = serde_json::from_str(website_data_json).unwrap();

    //     let mut generated_code = String::new();
    //     generated_code.push_str(
    //         "use serde::{Deserialize, Serialize};
    // use nucleo::Utf32Str;

    // #[derive(Serialize, Deserialize)]
    // struct WebsiteDataTitle<'a> {
    //     id: u64,
    //     title: &'a str,
    //     url: &'a str,
    //     tags: Vec<usize>
    // }
    // pub const WEBSITE_DATA_TITLE: [WebsiteDataTitle; ",
    //     );
    //     generated_code.push_str(&website_data.len().to_string());
    //     generated_code.push_str("] = [");

    //     for data in website_data.clone() {
    //         generated_code.push_str(&format!(
    //             "WebsiteData {{ id: {}, title: \"{}\", url: \"{}\", tags: {:?} }},",
    //             data.id,
    //             data.title.escape_default(),
    //             data.url,
    //             data.tags,
    //         ));
    //     }
    //     if website_data.len() != 0 {
    //         generated_code.remove(generated_code.len() - 1);
    //     }
    //     generated_code.push_str("];");
    //     let out_dir = std::env::var("OUT_DIR").unwrap();
    //     let dest_path = Path::new(&out_dir).join("generated_website_title_data.rs");
    //     fs::write(dest_path, generated_code).unwrap();
}

// fn process_tags_json() {
//     let tags_json = include_str!("../scraper/tags.json");

//     let tags_data: Vec<&str> = serde_json::from_str(tags_json).unwrap();

//     let mut generated_code = String::new();
//     generated_code.push_str("pub const TAGS: [&str; ");
//     generated_code.push_str(&tags_data.len().to_string());
//     generated_code.push_str("] = [");

//     for data in tags_data.clone() {
//         generated_code.push_str(&format!("\"{}\",", data));
//     }
//     if tags_data.len() != 0 {
//         generated_code.remove(generated_code.len() - 1);
//     }
//     generated_code.push_str("];");
//     // generated_code.push_str("\n");
//     // generated_code.push_str(r#"pub const WEBSITE_DATAUTF32STR: Utf32Str =Utf32Str::Ascii(b"#);
//     // for data in website_data.clone() {
//     //     generated_code.push_str(&format!("{:#?},", data.title.to_ascii_lowercase()));
//     // }
//     // if website_data.len() != 0 {
//     //     generated_code.remove(generated_code.len() - 1);
//     // }
//     // generated_code.push_str(r#");"#);
//     let out_dir = std::env::var("OUT_DIR").unwrap();
//     let dest_path = Path::new(&out_dir).join("generated_tags.rs");
//     fs::write(dest_path, generated_code).unwrap();
// }

fn main() {
    process_websites_json();
    // process_tags_json();
}
