use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::official::types::core::Pagination;

/// Wraps API responses which have the single field `data`.
/// Methods that make calls to endpoints that return this will unwrap it
/// and provide the value of `data` directly.
///
/// | [`Client`] Methods            | API Reference                |
/// | ----------------------------- | ---------------------------- |
/// | [`game`]                      | [Get Game Response]          |
/// | [`game_versions`]             | [Get Versions Response]      |
/// | [`game_version_types`]        | [Get Version Types Response] |
/// | [`categories`]                | [Get Categories Response]    |
/// | [`project`]                   | [Get Mod Response]           |
/// | [`projects`]                  | [Get Mods Response]          |
/// | [`featured_projects`]         | [Get Featured Mods Response] |
/// | [`project_description`]       | [String Response]            |
/// | [`project_file`]              | [Get Mod File Response]      |
/// | [`project_files_by_ids`]      | [Get Files Response]         |
/// | [`project_file_changelog`]    | [String Response]            |
/// | [`project_file_download_url`] | [String Response]            |
///
/// [`Client`]: crate::official::client::Client
/// [`game`]: crate::official::client::Client::game
/// [`game_versions`]: crate::official::client::Client::game_versions
/// [`game_version_types`]: crate::official::client::Client::game_version_types
/// [`categories`]: crate::official::client::Client::categories
/// [`project`]: crate::official::client::Client::project
/// [`projects`]: crate::official::client::Client::projects
/// [`featured_projects`]: crate::official::client::Client::featured_projects
/// [`project_description`]: crate::official::client::Client::project_description
/// [`project_file`]: crate::official::client::Client::project_file
/// [`project_files_by_ids`]: crate::official::client::Client::project_files_by_ids
/// [`project_file_changelog`]: crate::official::client::Client::project_file_changelog
/// [`project_file_download_url`]: crate::official::client::Client::project_file_download_url
///
/// [Get Game response]: https://docs.curseforge.com/#tocS_Get%20Game%20Response
/// [Get Versions Response]: https://docs.curseforge.com/#tocS_Get%20Versions%20Response
/// [Get Version Types Response]: https://docs.curseforge.com/#tocS_Get%20Version%20Types%20Response
/// [Get Categories Response]: https://docs.curseforge.com/#tocS_Get%20Categories%20Response
/// [Get Mod Response]: https://docs.curseforge.com/#tocS_Get%20Mod%20Response
/// [Get Mods Response]: https://docs.curseforge.com/#tocS_Get%20Mods%20Response
/// [Get Featured Mods Response]: https://docs.curseforge.com/#tocS_Get%20Featured%20Mods%20Response
/// [Get Mod File Response]: https://docs.curseforge.com/#tocS_Get%20Mod%20File%20Response
/// [Get Files Response]: https://docs.curseforge.com/#tocS_Get%20Files%20Response
/// [String Response]: https://docs.curseforge.com/#tocS_String%20Response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DataResponse<T> {
    pub data: T,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

impl<T> Deref for DataResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for DataResponse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Wraps API responses which have the fields `data` and `pagination`.
///
/// | [`Client`] Methods       | API Reference            |
/// | ------------------------ | ------------------------ |
/// | [`games`]                | [Get Games Response]     |
/// | [`games_iter`]           | [Get Games Response]     |
/// | [`search_projects`]      | [Search Mods Response]   |
/// | [`search_projects_iter`] | [Search Mods Response]   |
/// | [`project_files`]        | [Get Mod Files Response] |
/// | [`project_files_iter`]   | [Get Mod Files Response] |
///
/// [`Client`]: crate::official::client::Client
/// [`games`]: crate::official::client::Client::games
/// [`games_iter`]: crate::official::client::Client::games_iter
/// [`search_projects`]: crate::official::client::Client::search_projects
/// [`search_projects_iter`]: crate::official::client::Client::search_projects_iter
/// [`project_files`]: crate::official::client::Client::project_files
/// [`project_files_iter`]: crate::official::client::Client::project_files_iter
///
/// [Get Games Response]: https://docs.curseforge.com/#tocS_Get%20Games%20Response
/// [Search Mods Response]: https://docs.curseforge.com/#tocS_Search%20Mods%20Response
/// [Get Mod Files Response]: https://docs.curseforge.com/#tocS_Get%20Mod%20Files%20Response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PaginatedDataResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub(crate) bytes: Vec<u8>,
    pub(crate) value: T,
}

impl<T> ApiResponse<T> {
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn get_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn into_bytes_value(self) -> (Vec<u8>, T) {
        (self.bytes, self.value)
    }
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ApiResponse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub type ApiDataResult<T> = Result<ApiResponse<DataResponse<T>>, crate::Error>;
pub type ApiPageResult<T> = Result<ApiResponse<PaginatedDataResponse<T>>, crate::Error>;
