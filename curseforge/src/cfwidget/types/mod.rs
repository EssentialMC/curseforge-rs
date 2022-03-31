use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDownloads {
    pub monthly: usize,
    pub total: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectUrls {
    pub curseforge: String,
    pub project: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ReleaseType {
    Release,
    Beta,
    Alpha,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectMember {
    pub title: String,
    pub username: String,
    pub id: u32,
}
