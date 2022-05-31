//! Contains methods that take an [`isahc::HttpClient`] and make a request
//! to a CurseForge endpoint.

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::official::request::pagination::{
    GamesDelegate, GamesStream, ProjectFilesDelegate, ProjectFilesStream, ProjectSearchDelegate,
    ProjectSearchStream,
};
use crate::official::request::params::{
    several_body, CategoriesParams, FeaturedProjectsBody, GamesParams, ProjectFilesParams,
    ProjectSearchParams,
};
use crate::official::request::response::{DataResponse, PaginatedDataResponse};
use crate::official::types::{
    Category, FeaturedProjects, Game, GameVersionType, GameVersions, Project, ProjectFile,
};
use crate::Error;

/// This is the official CurseForge Core API base URL.
/// You must pass it to constructors explicitly.
pub const DEFAULT_API_BASE: &str = "https://api.curseforge.com/v1/";
/// The CurseForge API has a maximum limit of 10,000 results that can be
/// returned from any paginated request. Refer to the
/// [documentation](https://docs.curseforge.com/#pagination-limits) for more information.
pub const API_PAGINATION_RESULTS_LIMIT: usize = 10_000;

#[derive(Debug)]
pub struct ApiResponse<T> {
    bytes: Vec<u8>,
    value: T,
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

pub type ApiDataResult<T> = Result<ApiResponse<DataResponse<T>>, Error>;
pub type ApiPageResult<T> = Result<ApiResponse<PaginatedDataResponse<T>>, Error>;

macro_rules! endpoint {
    (
        $client:ident $method:ident
        uri: $base:ident / $path:literal,
        $(vars: [$($var:ident),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
    ) => {{
        use futures_lite::io::AsyncReadExt;

        #[allow(unused_mut)]
        let mut url = endpoint!(@uri, $base, $path $(, [$($var),*])?);

        $(url.set_query(Some(&serde_qs::to_string($params).unwrap()));)?

        let builder = isahc::Request::builder()
            .method(endpoint!(@str $method))
            .uri(url.as_str());
        let request = endpoint!(@build, builder $(, $body)?)?;

        let response = $client.send_async(request).await?;
        let mut bytes = Vec::new();

        response.into_body().read_to_end(&mut bytes).await.unwrap();

        match serde_json::from_slice(bytes.as_slice()) {
            Ok(value) => Ok(ApiResponse { bytes, value }),
            Err(error) => Err(Error::Parsing { error, bytes }),
        }
    }};
    (@uri, $base:ident, $path:literal) => {
        $base.join($path).unwrap()
    };
    (@uri, $base:ident, $path:literal, [$($var:ident),+]) => {
        $base.join(&format!($path, $($var),*)).unwrap()
    };
    (@build, $builder:ident) => {
        $builder.body(())
    };
    (@build, $builder:ident, $body:expr) => {
        $builder.body(serde_json::to_string($body).unwrap())
    };
    (@str GET) => {
        "GET"
    };
    (@str POST) => {
        "POST"
    };
}

/// <https://docs.curseforge.com/#get-game>
pub async fn game(
    client: &isahc::HttpClient,
    base: &url::Url,
    game_id: i32,
) -> ApiDataResult<Game> {
    endpoint! {
        client GET
        uri: base / "games/{}",
        vars: [game_id],
    }
}

/// <https://docs.curseforge.com/#get-games>
pub async fn games(
    client: &isahc::HttpClient,
    base: &url::Url,
    params: &GamesParams,
) -> ApiPageResult<Game> {
    endpoint! {
        client GET
        uri: base / "games",
        params: params,
    }
}

/// <https://docs.curseforge.com/#get-games>
pub fn games_iter<'cu, 'f>(
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    params: GamesParams,
) -> GamesStream<'cu, 'f> {
    GamesDelegate::new(client, base, params).into()
}

/// <https://docs.curseforge.com/#get-versions>
pub async fn game_versions(
    client: &isahc::HttpClient,
    base: &url::Url,
    game_id: i32,
) -> ApiDataResult<Vec<GameVersions>> {
    endpoint! {
        client GET
        uri: base / "games/{}/versions",
        vars: [game_id],
    }
}

/// <https://docs.curseforge.com/#get-version-types>
pub async fn game_version_types(
    client: &isahc::HttpClient,
    base: &url::Url,
    game_id: i32,
) -> ApiDataResult<Vec<GameVersionType>> {
    endpoint! {
        client GET
        uri: base / "games/{}/version-types",
        vars: [game_id],
    }
}

/// <https://docs.curseforge.com/#get-categories>
pub async fn categories(
    client: &isahc::HttpClient,
    base: &url::Url,
    params: &CategoriesParams,
) -> ApiDataResult<Vec<Category>> {
    endpoint! {
        client GET
        uri: base / "categories",
        params: params,
    }
}

/// <https://docs.curseforge.com/#search-mods>
pub async fn search_projects(
    client: &isahc::HttpClient,
    base: &url::Url,
    params: &ProjectSearchParams,
) -> ApiPageResult<Project> {
    endpoint! {
        client GET
        uri: base / "mods/search",
        params: params,
    }
}

/// <https://docs.curseforge.com/#search-mods>
///
/// This adheres to the limit of results defined by the
/// [documentation](https://docs.curseforge.com/#pagination-limits),
/// hardcoded by the constant [`API_PAGINATION_RESULTS_LIMIT`].
pub fn search_projects_iter<'cu, 'f>(
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    params: ProjectSearchParams,
) -> ProjectSearchStream<'cu, 'f> {
    ProjectSearchDelegate::new(client, base, params).into()
}

/// <https://docs.curseforge.com/#get-mod>
///
/// Renamed from `mod` to `project` because the former is a keyword, and the
/// API considers every "project" to be a "mod".
pub async fn project(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: i32,
) -> ApiDataResult<Project> {
    endpoint! {
        client GET
        uri: base / "mods/{}",
        vars: [project_id],
    }
}

/// <https://docs.curseforge.com/#get-mods>
pub async fn projects<I>(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_ids: I,
) -> ApiDataResult<Vec<Project>>
where
    I: IntoIterator<Item = i32>,
{
    endpoint! {
        client POST
        uri: base / "mods",
        body: &several_body!("modIds", i32, project_ids.into_iter()),
    }
}

/// <https://docs.curseforge.com/#get-featured-mods>
pub async fn featured_projects(
    client: &isahc::HttpClient,
    base: &url::Url,
    body: &FeaturedProjectsBody,
) -> ApiDataResult<FeaturedProjects> {
    endpoint! {
        client POST
        uri: base / "mods/featured",
        body: body,
    }
}

/// <https://docs.curseforge.com/#get-mod-description>
pub async fn project_description(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: i32,
) -> ApiDataResult<String> {
    endpoint! {
        client GET
        uri: base / "mods/{}/description",
        vars: [project_id],
    }
}

/// <https://docs.curseforge.com/#get-mod-file>
pub async fn project_file(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: i32,
    file_id: i32,
) -> ApiDataResult<ProjectFile> {
    endpoint! {
        client GET
        uri: base / "mods/{}/files/{}",
        vars: [project_id, file_id],
    }
}

/// Alternative method to [`project_file`] that eliminates the need
/// for a `project_id`. This uses [`project_files_by_ids`] and
/// returns the only item.
pub async fn project_file_by_id(
    client: &isahc::HttpClient,
    base: &url::Url,
    file_id: i32,
) -> ApiDataResult<ProjectFile> {
    project_files_by_ids(client, base, [file_id])
        .await
        .map(|mut r| ApiResponse {
            bytes: r.bytes,
            // Use of unwrap: if no item were present the bytes would be empty (parse error)
            value: DataResponse {
                data: r.value.data.pop().unwrap(),
                #[cfg(feature = "allow-unknown-fields")]
                other_fields: r.value.other_fields,
            },
        })
}

/// <https://docs.curseforge.com/#get-mod-files>
pub async fn project_files(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: i32,
    params: &ProjectFilesParams,
) -> ApiPageResult<ProjectFile> {
    endpoint! {
        client GET
        uri: base / "mods/{}/files",
        vars: [project_id],
        params: params,
    }
}

/// <https://docs.curseforge.com/#get-mod-files>
///
/// This adheres to the limit of results defined by the
/// [documentation](https://docs.curseforge.com/#pagination-limits),
/// hardcoded by the constant [`API_PAGINATION_RESULTS_LIMIT`].
pub fn project_files_iter<'cu, 'f>(
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    project_id: i32,
    params: ProjectFilesParams,
) -> ProjectFilesStream<'cu, 'f> {
    ProjectFilesDelegate::new(client, base, project_id, params).into()
}

/// <https://docs.curseforge.com/#get-files>
pub async fn project_files_by_ids<I>(
    client: &isahc::HttpClient,
    base: &url::Url,
    file_ids: I,
) -> ApiDataResult<Vec<ProjectFile>>
where
    I: IntoIterator<Item = i32>,
{
    endpoint! {
        client POST
        uri: base / "mods/files",
        body: &several_body!("fileIds", i32, file_ids.into_iter()),
    }
}

/// <https://docs.curseforge.com/#get-mod-file-changelog>
pub async fn project_file_changelog(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: i32,
    file_id: i32,
) -> ApiDataResult<String> {
    endpoint! {
        client GET
        uri: base / "mods/{}/files/{}/changelog",
        vars: [project_id, file_id],
    }
}

/// <https://docs.curseforge.com/#get-mod-file-download-url>
pub async fn project_file_download_url(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: i32,
    file_id: i32,
) -> ApiDataResult<String> {
    endpoint! {
        client GET
        uri: base / "mods/{}/files/{}/download-url",
        vars: [project_id, file_id],
    }
}
