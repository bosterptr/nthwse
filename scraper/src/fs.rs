use serde::de::DeserializeOwned;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub async fn save_to_json<T>(path: &str, data: Vec<T>) -> Result<(), std::io::Error>
where
    T: DeserializeOwned + serde::Serialize,
{
    let json_vec = serde_json::to_vec(&data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .open(path)
        .await?;
    file.write_all(&json_vec).await.expect("Failed to write to {path} file");
    let _ = file.flush();
    Ok(())
}
