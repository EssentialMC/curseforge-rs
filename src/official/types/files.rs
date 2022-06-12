use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::fixes::deserialize_nullable_string;
use super::projects::ModLoaderType;

/// <https://docs.curseforge.com/#tocS_File>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ProjectFile {
    pub id: i32,
    pub game_id: i32,
    #[serde(rename = "modId")]
    pub project_id: i32,
    pub is_available: bool,
    pub display_name: String,
    pub file_name: String,
    pub release_type: FileReleaseType,
    pub file_status: FileStatus,
    pub hashes: Vec<FileHash>,
    pub file_date: DateTime<Utc>,
    pub file_length: i64,
    pub download_count: i64,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    #[serde(default)]
    pub expose_as_alternative: bool,
    #[serde(default)]
    pub parent_project_file_id: Option<i32>,
    pub alternate_file_id: Option<i32>,
    pub is_server_pack: bool,
    #[serde(default)]
    pub server_pack_file_id: Option<i32>,
    pub file_fingerprint: i64,
    pub modules: Vec<FileModule>,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_FileIndex>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: i32,
    pub filename: String,
    pub release_type: FileReleaseType,
    pub game_version_type_id: Option<i32>,
    pub mod_loader: Option<ModLoaderType>,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_FileReleaseType>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_FileStatus>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_FileHash>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgorithm,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_HashAlgo>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HashAlgorithm {
    Sha1 = 1,
    Md5 = 2,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_SortableGameVersion>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SortableGameVersion {
    pub game_version_name: String,
    #[serde(deserialize_with = "deserialize_nullable_string")]
    pub game_version_padded: Option<String>,
    #[serde(deserialize_with = "deserialize_nullable_string")]
    pub game_version: Option<String>,
    pub game_version_release_date: DateTime<Utc>,
    pub game_version_type_id: Option<i32>,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_FileDependency>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FileDependency {
    #[serde(rename = "modId")]
    pub project_id: i32,
    pub relation_type: FileRelationType,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_FileRelationType>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_FileModule>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FileModule {
    name: String,
    fingerprint: i64,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}
