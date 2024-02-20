use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Write json error : {0}")]
    WriteJsonError(#[from] WriteJsonError),
    #[error("Read json error : {0}")]
    ReadJsonError(#[from] ReadJsonError),
}

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
