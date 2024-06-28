use crate::{config::CONFIG, extractor::ExtractedWebsite};
use tokio::{
    fs,
    io::{self, AsyncReadExt},
};

pub async fn load_website_from_disk(extracted_website: ExtractedWebsite) -> Result<String, ()> {
    let file = fs::File::open(format!(
        "{}/{}.html",
        CONFIG.nthw_raw_websites_path, extracted_website.id
    ))
    .await
    .expect("couldn't find the html file");
    let mut reader = io::BufReader::new(file);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .await
        .expect("couldn't read html");
    Ok(content)
}
