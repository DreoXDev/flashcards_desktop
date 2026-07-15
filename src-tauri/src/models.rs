use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckImport {
    pub schema_version: i64,
    pub deck: DeckDefinition,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub subject: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub cards: Vec<FlashcardDefinition>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(
    tag = "type",
    rename_all = "lowercase",
    rename_all_fields = "camelCase"
)]
pub enum FlashcardDefinition {
    Open {
        id: String,
        question: String,
        answer: String,
        #[serde(default)]
        tags: Vec<String>,
        source: Option<String>,
        notes: Option<String>,
    },
    Closed {
        id: String,
        question: String,
        options: Vec<ClosedCardOption>,
        correct_option_ids: Vec<String>,
        explanation: Option<String>,
        #[serde(default)]
        tags: Vec<String>,
        source: Option<String>,
        notes: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClosedCardOption {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<String>,
    pub card_count: i64,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentDeck {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<String>,
    pub card_count: i64,
    pub last_studied_at: String,
    pub last_known_count: i64,
    pub last_unknown_count: i64,
    pub last_unknown_card_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<String>,
    pub cards: Vec<Flashcard>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Flashcard {
    pub id: String,
    pub card_type: String,
    pub position: i64,
    pub question: String,
    pub answer: Option<String>,
    pub options: Vec<ClosedCardOption>,
    pub correct_option_ids: Vec<String>,
    pub explanation: Option<String>,
    pub tags: Vec<String>,
    pub source: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub deck_id: String,
    pub deck_name: String,
    pub card_count: usize,
    pub replaced: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveStudyHistoryRequest {
    pub deck_id: String,
    pub last_known_count: i64,
    pub last_unknown_count: i64,
    pub last_unknown_card_ids: Vec<String>,
}
