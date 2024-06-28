use crate::extractor::ExtractedWebsite;
use crate::globals::TAKEN_PORTS;
use crate::processor::process_website;
use deadpool::managed::Metrics;
use deadpool::managed::{Manager, Pool, PoolConfig, RecycleResult};
use fantoccini::ClientBuilder;
use serde_json::json;
use std::process::Command;
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Fantoccini error: {0}")]
    Fantoccini(#[from] fantoccini::error::NewSessionError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct PoolManager {
    base_port: u16,
}

pub struct Scraper {
    client: fantoccini::Client,
    driver: std::process::Child,
}
impl Scraper {
    pub async fn process_website(&self, website: &ExtractedWebsite) -> () {
        process_website(website, &self.client).await
    }
}

impl Drop for Scraper {
    fn drop(&mut self) {
        let _ = self.driver.kill();
        let _ = self.driver.wait();
    }
}
fn get_first_available_port(start: u16) -> u16 {
    let mut current = start;
    loop {
        if !TAKEN_PORTS.lock().unwrap().contains(&current) {
            return current;
        }
        current += 1;
    }
}
fn get_next_port(base_port: u16) -> u16 {
    let first_avaible_port = get_first_available_port(base_port);
    TAKEN_PORTS.lock().unwrap().push(first_avaible_port.into());
    debug!("first_avaible_port {first_avaible_port}");
    first_avaible_port
}
impl PoolManager {
    fn new(base_port: u16) -> Self {
        Self { base_port }
    }
}
impl Manager for PoolManager {
    type Type = Scraper;
    type Error = PoolError;

    async fn create(&self) -> Result<Scraper, PoolError> {
        let port = get_next_port(self.base_port);
        let binding = which::which("firefox").expect("Firefox not found on this system.");
        let firefox_path = binding
            .to_str()
            .expect("Firefox not found on this system.")
            .to_string();
        let mut capabilities = serde_json::map::Map::new();
        let firefox_options = json!({
            "prefs": {
                "dom.disable_open_during_load": true,
                "dom.popup_maximum": 0
            },
            "binary": firefox_path
        });
        capabilities.insert("moz:firefoxOptions".to_string(), firefox_options);
        let driver = Command::new(format!("geckodriver"))
            .arg(&format!("--port={port}"))
            .spawn()
            .expect("Failed to execute command");
        let client: fantoccini::Client = ClientBuilder::native()
            .capabilities(capabilities.clone())
            .connect(&format!("http://localhost:{port}"))
            .await
            .expect("couldn't connect to a web driver");
        Ok(Scraper { client, driver })
    }

    async fn recycle(&self, _: &mut Scraper, _: &Metrics) -> RecycleResult<Self::Error> {
        // Booo hooo, you can't get the &mut self
        // TAKEN_PORTS.lock().unwrap().retain(|&x| x != scraper.port);
        Ok(())
    }

    fn detach(&self, _obj: &mut Self::Type) {}
}
pub fn create_pool(base_port: u16, pool_size: usize) -> Pool<PoolManager> {
    let manager = PoolManager::new(base_port);
    let pool_config = PoolConfig::new(pool_size);
    Pool::builder(manager).config(pool_config).build().unwrap()
}
