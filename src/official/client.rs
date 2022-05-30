//! Contains the [`Client`] structure whose methods are used to make
//! requests to the remote API.

use super::request::pagination::{
    GamesDelegate, GamesStream, ProjectFilesDelegate, ProjectFilesStream, ProjectSearchDelegate,
    ProjectSearchStream,
};
use super::request::params::{
    several_body, CategoriesParams, FeaturedProjectsBody, GamesParams, ProjectFilesParams,
    ProjectSearchParams,
};
use super::request::response::{DataResponse, PaginatedDataResponse};
use super::types::{
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

macro_rules! endpoint {
    (
        $self:ident $method:ident $uri:literal,
        $(vars: [$($var:ident),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
        into: $into:path,
    ) => {{
        use futures_lite::io::AsyncReadExt;

        #[allow(unused_mut)]
        let mut url = endpoint!(@uri, $self, $uri $(, [$($var),*])?);

        $(url.set_query(Some(&serde_qs::to_string($params).unwrap()));)?

        let builder = isahc::Request::builder().method(endpoint!(@str $method)).uri(url.as_str());
        let request = endpoint!(@build, builder $(, $body)?)?;

        let response = $self.inner.send_async(request).await?;
        let (head, mut body) = response.into_parts();

        // let mut bytes = Vec::with_capacity(
        //     head.headers
        //         .get("content-length")
        //         .unwrap()
        //         .to_str()
        //         .unwrap()
        //         .parse()
        //         .unwrap(),
        // );
        let mut bytes = Vec::new();

        body.read_to_end(&mut bytes).await.unwrap();

        let value: $into = serde_json::from_slice(bytes.as_slice())?;

        (head, bytes, value)
    }};
    (@uri, $self:ident, $uri:literal) => {
        $self.base.join($uri).unwrap()
    };
    (@uri, $self:ident, $uri:literal, [$($var:ident),+]) => {
        $self.base.join(&format!($uri, $($var),*)).unwrap()
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

/// This structure wraps a [`surf::Client`] and implements methods to easily
/// make requests to various API endpoints. The default [`Self::new`]
/// constructor should be used if you need basic functionality, but if you just
/// need to add extra headers or the like use [`Self::with_config`] and provide
/// a custom [`surf::Config`].
#[derive(Clone, Debug)]
pub struct Client {
    inner: isahc::HttpClient,
    base: url::Url,
}

impl Client {
    /// Constructs a client for the CurseForge Core API, given an
    /// API base URL (use [`DEFAULT_API_BASE`] if not using a proxy)
    /// and an optional token for authentication (required without a proxy).
    pub fn new<U>(base: U, token: Option<String>) -> Result<Self, Error>
    where
        U: AsRef<str>,
    {
        let mut builder = isahc::HttpClient::builder();

        builder = builder.default_header("accept", "application/json");

        if let Some(token) = token {
            builder = builder.default_header("x-api-key", token);
        }

        let base = url::Url::parse(base.as_ref())?;

        if base.cannot_be_a_base() {
            Err(Error::BadBaseUrl)?;
        }

        Ok(Self {
            inner: builder.build()?,
            base,
        })
    }

    /// <https://docs.curseforge.com/#get-game>
    pub async fn game(&self, game_id: i32) -> Result<Game, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "games/{}",
            vars: [game_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-games>
    pub async fn games(&self, params: &GamesParams) -> Result<PaginatedDataResponse<Game>, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "games",
            params: params,
            into: PaginatedDataResponse<_>,
        };

        Ok(value)
    }

    /// <https://docs.curseforge.com/#get-games>
    pub fn games_iter(&self, params: GamesParams) -> GamesStream {
        GamesDelegate::new(self, params).into()
    }

    /// <https://docs.curseforge.com/#get-versions>
    pub async fn game_versions(&self, game_id: i32) -> Result<Vec<GameVersions>, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "games/{}/versions",
            vars: [game_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-version-types>
    pub async fn game_version_types(&self, game_id: i32) -> Result<Vec<GameVersionType>, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "games/{}/version-types",
            vars: [game_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-categories>
    pub async fn categories(&self, params: &CategoriesParams) -> Result<Vec<Category>, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "categories",
            params: params,
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#search-mods>
    pub async fn search_projects(
        &self,
        params: &ProjectSearchParams,
    ) -> Result<PaginatedDataResponse<Project>, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/search",
            params: params,
            into: PaginatedDataResponse<_>,
        };

        Ok(value)
    }

    /// <https://docs.curseforge.com/#search-mods>
    ///
    /// This adheres to the limit of results defined by the
    /// [documentation](https://docs.curseforge.com/#pagination-limits),
    /// hardcoded by the constant [`API_PAGINATION_RESULTS_LIMIT`].
    pub fn search_projects_iter(&self, params: ProjectSearchParams) -> ProjectSearchStream {
        ProjectSearchDelegate::new(self, params).into()
    }

    /// <https://docs.curseforge.com/#get-mod>
    ///
    /// Renamed from `mod` to `project` because the former is a keyword, and the
    /// API considers every "project" to be a "mod".
    pub async fn project(&self, project_id: i32) -> Result<Project, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/{}",
            vars: [project_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mods>
    pub async fn projects<I>(&self, project_ids: I) -> Result<Vec<Project>, Error>
    where
        I: IntoIterator<Item = i32>,
    {
        let (_response, _bytes, value) = endpoint! {
            self POST "mods",
            body: &several_body!("modIds", i32, project_ids.into_iter()),
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-featured-mods>
    pub async fn featured_projects(
        &self,
        body: &FeaturedProjectsBody,
    ) -> Result<FeaturedProjects, Error> {
        let (_response, _bytes, value) = endpoint! {
            self POST "mods/featured",
            body: body,
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-description>
    pub async fn project_description(&self, project_id: i32) -> Result<String, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/{}/description",
            vars: [project_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-file>
    pub async fn project_file(&self, project_id: i32, file_id: i32) -> Result<ProjectFile, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/{}/files/{}",
            vars: [project_id, file_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// Alternative method to [`Self::project_file`] that eliminates the need
    /// for a `project_id`. This uses [`Self::project_files_by_ids`] and
    /// returns the only item.
    pub async fn project_file_by_id(&self, file_id: i32) -> Result<ProjectFile, Error> {
        Ok(self.project_files_by_ids([file_id]).await?.pop().unwrap())
    }

    /// <https://docs.curseforge.com/#get-mod-files>
    pub async fn project_files(
        &self,
        project_id: i32,
        params: &ProjectFilesParams,
    ) -> Result<PaginatedDataResponse<ProjectFile>, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/{}/files",
            vars: [project_id],
            params: params,
            into: PaginatedDataResponse<_>,
        };

        Ok(value)
    }

    /// <https://docs.curseforge.com/#get-mod-files>
    ///
    /// This adheres to the limit of results defined by the
    /// [documentation](https://docs.curseforge.com/#pagination-limits),
    /// hardcoded by the constant [`API_PAGINATION_RESULTS_LIMIT`].
    pub fn project_files_iter(
        &self,
        project_id: i32,
        params: ProjectFilesParams,
    ) -> ProjectFilesStream {
        ProjectFilesDelegate::new(self, project_id, params).into()
    }

    /// <https://docs.curseforge.com/#get-files>
    pub async fn project_files_by_ids<I>(&self, file_ids: I) -> Result<Vec<ProjectFile>, Error>
    where
        I: IntoIterator<Item = i32>,
    {
        let (_response, _bytes, value) = endpoint! {
            self POST "mods/files",
            body: &several_body!("fileIds", i32, file_ids.into_iter()),
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-file-changelog>
    pub async fn project_file_changelog(
        &self,
        project_id: i32,
        file_id: i32,
    ) -> Result<String, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/{}/files/{}/changelog",
            vars: [project_id, file_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-file-download-url>
    pub async fn project_file_download_url(
        &self,
        project_id: i32,
        file_id: i32,
    ) -> Result<String, Error> {
        let (_response, _bytes, value) = endpoint! {
            self GET "mods/{}/files/{}/download-url",
            vars: [project_id, file_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }
}
