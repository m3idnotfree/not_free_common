use std::path::PathBuf;

use serde::de;
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
};

use crate::err::{ReadJsonError, WriteJsonError};

// pub async fn write_json<T: serde::Serialize>(path: PathBuf, content: T) -> Result<(), CommonError> {
pub async fn write_json<T: serde::Serialize>(
    path: PathBuf,
    content: T,
) -> Result<(), WriteJsonError> {
    let file = File::create(&path).await?;
    let mut writer = BufWriter::new(file);
    let res = serde_json::to_string_pretty(&content)?;
    writer.write_all(res.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn read_json<T: de::DeserializeOwned>(path: PathBuf) -> Result<T, ReadJsonError> {
    let file = fs::read_to_string(&path).await?;
    let result: T = serde_json::from_str(&file)?;

    Ok(result)
}
