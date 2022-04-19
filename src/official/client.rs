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

/// This is the official CurseForge Core API base URL.
/// You must pass it to constructors explicitly.
pub const DEFAULT_API_BASE: &str = "https://api.curseforge.com/v1/";
pub const API_PAGINATION_RESULTS_LIMIT: usize = 10_000;

macro_rules! endpoint {
    (
        $($subj_frag:ident).+ $method:ident $uri:literal,
        $(vars: [$($var:ident),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
        into: $into:path,
    ) => {{
        #[allow(unused_mut)]
        let mut request = endpoint!(@init, $($subj_frag).*, $method, $uri $(, [$($var),*])?);
        $(request = request.query($params)?;)?
        $(request = request.body_json($body)?;)?
        let request = request.build();
        let mut response = $($subj_frag).*.send(request).await?;
        let bytes = response.body_bytes().await?;
        let value: $into = serde_json::from_slice(bytes.as_slice())?;

        (response, bytes, value)
    }};
    (@init, $($subj_frag:ident).+, $method:ident, $uri:literal) => {
        $($subj_frag).*.$method($uri)
    };
    (@init, $($subj_frag:ident).+, $method:ident, $uri:literal, [$($var:ident),+]) => {
        $($subj_frag).*.$method(&format!($uri, $($var),*))
    };
}

/// This structure wraps a [`surf::Client`] and implements methods to easily
/// make requests to various API endpoints. The default [`Self::new`]
/// constructor should be used if you need basic functionality, but if you just
/// need to add extra headers or the like use [`Self::with_config`] and provide
/// a custom [`surf::Config`].
#[derive(Clone, Debug)]
pub struct Client {
    inner: surf::Client,
    #[allow(dead_code)]
    base: String,
}

impl Client {
    /// Constructs a client for the CurseForge Core API, given an
    /// API base URL (use [`DEFAULT_API_BASE`] if not using a proxy)
    /// and an optional token for authentication (required without a proxy).
    pub fn new<U>(base: U, token: Option<String>) -> surf::Result<Self>
    where
        U: AsRef<str>,
    {
        let mut config = surf::Config::new();

        if let Some(token) = token {
            config = config.add_header("x-api-key", token)?;
        }

        Self::with_config(base, config)
    }

    /// Constructs a client with a provided [`surf::Config`]. The API base URL
    /// is still required to be passed, and you must add a token manually with
    /// the header `x-api-key` if the base URL you choose requires (not an
    /// open proxy).
    pub fn with_config<U>(base: U, mut config: surf::Config) -> surf::Result<Self>
    where
        U: AsRef<str>,
    {
        config = config.add_header("Accept", "application/json")?;
        config = config.set_base_url(surf::Url::parse(base.as_ref())?);

        Ok(Self {
            inner: config.try_into()?,
            base: base.as_ref().to_owned(),
        })
    }

    /// <https://docs.curseforge.com/#get-game>
    pub async fn game(&self, game_id: i32) -> surf::Result<Game> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "games/{}",
            vars: [game_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-games>
    pub async fn games(&self, params: &GamesParams) -> surf::Result<PaginatedDataResponse<Game>> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "games",
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
    pub async fn game_versions(&self, game_id: i32) -> surf::Result<Vec<GameVersions>> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "games/{}/versions",
            vars: [game_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-version-types>
    pub async fn game_version_types(&self, game_id: i32) -> surf::Result<Vec<GameVersionType>> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "games/{}/version-types",
            vars: [game_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-categories>
    pub async fn categories(&self, params: &CategoriesParams) -> surf::Result<Vec<Category>> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "categories",
            params: params,
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#search-mods>
    pub async fn search_projects(
        &self,
        params: &ProjectSearchParams,
    ) -> surf::Result<PaginatedDataResponse<Project>> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/search",
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
    pub async fn project(&self, project_id: i32) -> surf::Result<Project> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/{}",
            vars: [project_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mods>
    pub async fn projects<I>(&self, project_ids: I) -> surf::Result<Vec<Project>>
    where
        I: IntoIterator<Item = i32>,
    {
        let (_response, _bytes, value) = endpoint! {
            self.inner post "mods",
            body: &several_body!("modIds", i32, project_ids.into_iter()),
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-featured-mods>
    pub async fn featured_projects(
        &self,
        body: &FeaturedProjectsBody,
    ) -> surf::Result<FeaturedProjects> {
        let (_response, _bytes, value) = endpoint! {
            self.inner post "mods/featured",
            body: body,
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-description>
    pub async fn project_description(&self, project_id: i32) -> surf::Result<String> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/{}/description",
            vars: [project_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-file>
    pub async fn project_file(&self, project_id: i32, file_id: i32) -> surf::Result<ProjectFile> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/{}/files/{}",
            vars: [project_id, file_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }

    /// <https://docs.curseforge.com/#get-mod-files>
    pub async fn project_files(
        &self,
        project_id: i32,
        params: &ProjectFilesParams,
    ) -> surf::Result<PaginatedDataResponse<ProjectFile>> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/{}/files",
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
    pub async fn project_files_by_ids<I>(&self, file_ids: I) -> surf::Result<Vec<ProjectFile>>
    where
        I: IntoIterator<Item = i32>,
    {
        let (_response, _bytes, value) = endpoint! {
            self.inner post "mods/files",
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
    ) -> surf::Result<String> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/{}/files/{}/changelog",
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
    ) -> surf::Result<String> {
        let (_response, _bytes, value) = endpoint! {
            self.inner get "mods/{}/files/{}/download-url",
            vars: [project_id, file_id],
            into: DataResponse<_>,
        };

        Ok(value.data)
    }
}
