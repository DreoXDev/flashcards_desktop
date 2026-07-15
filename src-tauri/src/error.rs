use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorPayload {
    pub code: String,
    pub message: String,
    pub details: Vec<String>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] libsql::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Invalid JSON")]
    JsonParse(#[from] serde_json::Error),
    #[error("Validation failed")]
    Validation(Vec<String>),
    #[error("Deck not found")]
    NotFound(String),
    #[error("Deck already exists")]
    Conflict(String),
    #[error("Configuration error")]
    Configuration(String),
}

impl AppError {
    pub fn payload(&self) -> AppErrorPayload {
        match self {
            AppError::Database(_) => AppErrorPayload {
                code: "DATABASE_ERROR".into(),
                message: "A database error occurred.".into(),
                details: vec![self.to_string()],
            },
            AppError::Io(_) => AppErrorPayload {
                code: "IO_ERROR".into(),
                message: "Could not read the selected file.".into(),
                details: vec![self.to_string()],
            },
            AppError::JsonParse(err) => AppErrorPayload {
                code: "DECK_JSON_PARSE_FAILED".into(),
                message: "The selected file is not valid JSON.".into(),
                details: vec![err.to_string()],
            },
            AppError::Validation(details) => AppErrorPayload {
                code: "DECK_VALIDATION_FAILED".into(),
                message: format!("The deck contains {} validation error(s).", details.len()),
                details: details.clone(),
            },
            AppError::NotFound(id) => AppErrorPayload {
                code: "DECK_NOT_FOUND".into(),
                message: "Deck not found.".into(),
                details: vec![format!("No deck exists with id \"{id}\".")],
            },
            AppError::Conflict(id) => AppErrorPayload {
                code: "DECK_CONFLICT".into(),
                message: "A deck with this ID already exists.".into(),
                details: vec![format!("Deck id \"{id}\" is already present.")],
            },
            AppError::Configuration(message) => AppErrorPayload {
                code: "CONFIGURATION_ERROR".into(),
                message: "Database configuration is invalid.".into(),
                details: vec![message.clone()],
            },
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.payload().serialize(serializer)
    }
}

pub type AppResult<T> = Result<T, AppError>;
