use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::categories::Category;
use super::files::{FileIndex, ProjectFile};
use super::fixes::deserialize_nullable_string;

/// <https://docs.curseforge.com/#tocS_ModLoaderType>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_Mod>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Project {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub links: ProjectLinks,
    pub summary: String,
    pub status: ProjectStatus,
    pub download_count: f64,
    pub is_featured: bool,
    pub primary_category_id: u32,
    pub categories: Vec<Category>,
    pub class_id: Option<i32>,
    pub authors: Vec<ProjectAuthor>,
    pub logo: Option<ProjectAsset>,
    pub screenshots: Vec<ProjectAsset>,
    pub main_file_id: i32,
    pub latest_files: Vec<ProjectFile>,
    pub latest_files_indexes: Vec<FileIndex>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub date_released: DateTime<Utc>,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: i32,
    pub is_available: bool,
    pub thumbs_up_count: Option<i64>,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_ModLinks>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ProjectLinks {
    pub website_url: String,
    #[serde(deserialize_with = "deserialize_nullable_string")]
    pub wiki_url: Option<String>,
    #[serde(deserialize_with = "deserialize_nullable_string")]
    pub issues_url: Option<String>,
    #[serde(deserialize_with = "deserialize_nullable_string")]
    pub source_url: Option<String>,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_ModLinks>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ProjectStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_ModAuthor>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ProjectAuthor {
    pub id: i32,
    pub name: String,
    pub url: String,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_ModAsset>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ProjectAsset {
    pub id: i32,
    #[serde(rename = "modId")]
    pub project_id: i32,
    pub title: String,
    #[serde(deserialize_with = "deserialize_nullable_string")]
    pub description: Option<String>,
    pub thumbnail_url: String,
    pub url: String,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_FeaturedModsResponse>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FeaturedProjects {
    pub featured: Vec<Project>,
    pub popular: Vec<Project>,
    pub recently_updated: Vec<Project>,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}
