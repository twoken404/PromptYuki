use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub id: String,
    pub title: String,
    pub content: String,
    pub description: Option<String>,
    pub variables: Vec<Variable>,
    pub tags: Vec<String>,
    pub folder_id: Option<String>,
    pub is_favorite: bool,
    pub usage_count: i32,
    pub rating: Option<i32>,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub var_type: String,
    pub default_value: Option<serde_json::Value>,
    pub options: Option<Vec<String>>,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub created_at: String,
    pub updated_at: String,
    pub last_used_at: Option<String>,
    pub word_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<String>,
    pub sort_order: i32,
}

pub fn get_database_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".promptyuki");
    path.push("prompts.db");
    path
}
