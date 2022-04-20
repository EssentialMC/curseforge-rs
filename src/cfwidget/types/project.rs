use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub game: String,
    #[serde(rename = "type")]
    pub release_type: String,
    pub urls: ProjectUrls,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub downloads: ProjectDownloads,
    pub license: String,
    pub donate: String,
    pub categories: Vec<String>,
    pub members: Vec<ProjectMember>,
    pub links: Vec<String>,
    pub files: Vec<ProjectFile>,
    pub versions: HashMap<String, ProjectFile>,
    pub download: ProjectFile,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct ProjectFile {
    pub id: u32,
    pub url: String,
    pub display: String,
    pub name: String,
    pub quality: ReleaseType,
    #[serde(rename = "version")]
    pub game_version: String,
    pub filesize: usize,
    // Can be "1.18.1" and "Forge", maybe this should be an enum?
    pub versions: Vec<String>,
    pub downloads: usize,
    pub uploaded_at: DateTime<Utc>,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct ProjectDownloads {
    pub monthly: usize,
    pub total: usize,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct ProjectUrls {
    pub curseforge: String,
    pub project: String,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReleaseType {
    Release,
    Beta,
    Alpha,
    #[cfg(feature = "unknown-fields")]
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct ProjectMember {
    pub title: String,
    pub username: String,
    pub id: u32,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}
