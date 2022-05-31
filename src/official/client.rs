//! Contains the [`Client`] structure whose methods are used to make
//! requests to the remote API.

use crate::official::request::{
    CategoriesParams, FeaturedProjectsBody, GamesDelegate, GamesParams, GamesStream,
    PaginatedDataResponse, ProjectFilesDelegate, ProjectFilesParams, ProjectFilesStream,
    ProjectSearchDelegate, ProjectSearchParams, ProjectSearchStream,
};
use crate::official::types::{
    Category, FeaturedProjects, Game, GameVersionType, GameVersions, Project, ProjectFile,
};
use crate::official::endpoints as e;
use crate::Error;

/// This is the official CurseForge Core API base URL.
/// You must pass it to constructors explicitly.
pub const DEFAULT_API_BASE: &str = "https://api.curseforge.com/v1/";
/// The CurseForge API has a maximum limit of 10,000 results that can be
/// returned from any paginated request. Refer to the
/// [documentation](https://docs.curseforge.com/#pagination-limits) for more information.
pub const API_PAGINATION_RESULTS_LIMIT: usize = 10_000;

/// This structure wraps an [`isahc::HttpClient`] and implements methods to
/// easily make requests to various API endpoints.
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

        builder = builder.default_header("content-type", "application/json");
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

    /// [`e::game`]
    pub async fn game(&self, game_id: i32) -> Result<Game, Error> {
        Ok(e::game(&self.inner, &self.base, game_id)
            .await?
            .into_value()
            .data)
    }

    /// [`e::games`]
    pub async fn games(&self, params: &GamesParams) -> Result<PaginatedDataResponse<Game>, Error> {
        Ok(e::games(&self.inner, &self.base, params)
            .await?
            .into_value())
    }

    /// [`e::games_iter`]
    pub fn games_iter(&self, params: GamesParams) -> GamesStream {
        GamesDelegate::new(&self.inner, &self.base, params).into()
    }

    /// [`e::game_versions`]
    pub async fn game_versions(&self, game_id: i32) -> Result<Vec<GameVersions>, Error> {
        Ok(e::game_versions(&self.inner, &self.base, game_id)
            .await?
            .into_value()
            .data)
    }

    /// [`e::game_version_types`]
    pub async fn game_version_types(&self, game_id: i32) -> Result<Vec<GameVersionType>, Error> {
        Ok(e::game_version_types(&self.inner, &self.base, game_id)
            .await?
            .into_value()
            .data)
    }

    /// [`e::categories`]
    pub async fn categories(&self, params: &CategoriesParams) -> Result<Vec<Category>, Error> {
        Ok(e::categories(&self.inner, &self.base, params)
            .await?
            .into_value()
            .data)
    }

    /// [`e::search_projects`]
    pub async fn search_projects(
        &self,
        params: &ProjectSearchParams,
    ) -> Result<PaginatedDataResponse<Project>, Error> {
        Ok(e::search_projects(&self.inner, &self.base, params)
            .await?
            .into_value())
    }

    /// [`e::search_projects_iter`]
    pub fn search_projects_iter(&self, params: ProjectSearchParams) -> ProjectSearchStream {
        ProjectSearchDelegate::new(&self.inner, &self.base, params).into()
    }

    /// [`e::project`]
    pub async fn project(&self, project_id: i32) -> Result<Project, Error> {
        Ok(e::project(&self.inner, &self.base, project_id)
            .await?
            .into_value()
            .data)
    }

    /// [`e::projects`]
    pub async fn projects<I>(&self, project_ids: I) -> Result<Vec<Project>, Error>
    where
        I: IntoIterator<Item = i32>,
    {
        Ok(e::projects(&self.inner, &self.base, project_ids)
            .await?
            .into_value()
            .data)
    }

    /// [`e::featured_projects`]
    pub async fn featured_projects(
        &self,
        body: &FeaturedProjectsBody,
    ) -> Result<FeaturedProjects, Error> {
        Ok(e::featured_projects(&self.inner, &self.base, body)
            .await?
            .into_value()
            .data)
    }

    /// [`e::project_description`]
    pub async fn project_description(&self, project_id: i32) -> Result<String, Error> {
        Ok(e::project_description(&self.inner, &self.base, project_id)
            .await?
            .into_value()
            .data)
    }

    /// [`e::project_file`]
    pub async fn project_file(&self, project_id: i32, file_id: i32) -> Result<ProjectFile, Error> {
        Ok(
            e::project_file(&self.inner, &self.base, project_id, file_id)
                .await?
                .into_value()
                .data,
        )
    }

    /// [`e::project_file_by_id`]
    pub async fn project_file_by_id(&self, file_id: i32) -> Result<ProjectFile, Error> {
        Ok(e::project_files_by_ids(&self.inner, &self.base, [file_id])
            .await?
            .into_value()
            .pop()
            .unwrap())
    }

    /// [`e::project_files`]
    pub async fn project_files(
        &self,
        project_id: i32,
        params: &ProjectFilesParams,
    ) -> Result<PaginatedDataResponse<ProjectFile>, Error> {
        Ok(
            e::project_files(&self.inner, &self.base, project_id, params)
                .await?
                .into_value(),
        )
    }

    /// [`e::project_files_iter`]
    pub fn project_files_iter(
        &self,
        project_id: i32,
        params: ProjectFilesParams,
    ) -> ProjectFilesStream {
        ProjectFilesDelegate::new(&self.inner, &self.base, project_id, params).into()
    }

    /// [`e::project_files_by_ids`]
    pub async fn project_files_by_ids<I>(&self, file_ids: I) -> Result<Vec<ProjectFile>, Error>
    where
        I: IntoIterator<Item = i32>,
    {
        Ok(e::project_files_by_ids(&self.inner, &self.base, file_ids)
            .await?
            .into_value()
            .data)
    }

    /// [`e::project_file_changelog`]
    pub async fn project_file_changelog(
        &self,
        project_id: i32,
        file_id: i32,
    ) -> Result<String, Error> {
        Ok(
            e::project_file_changelog(&self.inner, &self.base, project_id, file_id)
                .await?
                .into_value()
                .data,
        )
    }

    /// [`e::project_file_download_url`]
    pub async fn project_file_download_url(
        &self,
        project_id: i32,
        file_id: i32,
    ) -> Result<String, Error> {
        Ok(
            e::project_file_download_url(&self.inner, &self.base, project_id, file_id)
                .await?
                .into_value()
                .data,
        )
    }
}
