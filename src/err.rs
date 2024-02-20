use serde::Serialize;

// A custom error type that represents all possible in our command
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("Failed to read file: {0}")]
    // Io(#[from] std::io::Error),
    // #[error("File is not valid utf8: {0}")]
    // Utf8(#[from] std::string::FromUtf8Error),
    // #[error("Serde to string: {0}")]
    // ParseToJson(#[from] serde_json::Error),
    // #[error("Reqwest post: {0}")]
    // PostRewest(#[from] reqwest::Error),
    // #[error("Url parse Error: {0}")]
    // ParseUrl(#[from] url::ParseError),
    #[error("Write json error : {0}")]
    WriteJsonError(#[from] WriteJsonError),
    #[error("Read json error : {0}")]
    ReadJsonError(#[from] ReadJsonError),
}

// we must also implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
#[derive(Debug, thiserror::Error)]
pub enum WriteJsonError {
    #[error("Failed to open file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde to string: {0}")]
    ParseToJson(#[from] serde_json::Error),
}

impl Serialize for WriteJsonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
#[derive(Debug, thiserror::Error)]
pub enum ReadJsonError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde from string: {0}")]
    ParseToJson(#[from] serde_json::Error),
}

impl Serialize for ReadJsonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
