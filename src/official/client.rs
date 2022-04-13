use async_trait::async_trait;
use awaur::paginator::{PaginatedStream, PaginationDelegate};

use super::request::body::request_several_body;
use super::request::params::{CategoriesParams, GamesParams, ProjectFilesParams, SearchParams};
use super::request::response::{DataResponse, PaginatedDataResponse};
use super::types::{Category, File, Game, GameVersionType, GameVersions, Pagination, Project};

/// This is the official CurseForge Core API base URL.
/// You must pass it to constructors explicitly.
pub const DEFAULT_API_BASE: &str = "https://api.curseforge.com/v1/";
pub const API_PAGINATION_RESULTS_LIMIT: usize = 10_000;

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

    /// Constructs a client with a provided [`surf::Config`].
    /// The API base URL is still required to be passed.
    pub fn with_config<U>(base: U, mut config: surf::Config) -> surf::Result<Self>
    where
        U: AsRef<str>,
    {
        config = config.set_base_url(surf::Url::parse(base.as_ref())?);

        Ok(Self {
            inner: config.try_into()?,
            base: base.as_ref().to_owned(),
        })
    }

    /// <https://docs.curseforge.com/#get-game>
    pub async fn game(&self, game_id: i32) -> surf::Result<Game> {
        let response = self
            .inner
            .get(&format!("games/{}", game_id))
            .recv_bytes()
            .await?;

        let response: DataResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-games>
    pub async fn games(&self, params: &GamesParams) -> surf::Result<PaginatedDataResponse<Game>> {
        let response = self
            .inner
            .get(&format!("games?{}", params.to_query_string()))
            .recv_bytes()
            .await?;

        let response = serde_json::from_slice(response.as_slice())?;

        Ok(response)
    }

    /// <https://docs.curseforge.com/#get-versions>
    pub async fn game_versions(&self, game_id: i32) -> surf::Result<Vec<GameVersions>> {
        let response = self
            .inner
            .get(&format!("games/{}/versions", game_id))
            .recv_bytes()
            .await?;

        let response: DataResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-version-types>
    pub async fn game_version_types(&self, game_id: i32) -> surf::Result<Vec<GameVersionType>> {
        let response = self
            .inner
            .get(&format!("games/{}/version-types", game_id))
            .recv_bytes()
            .await?;

        let response: DataResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-categories>
    pub async fn categories(&self, params: &CategoriesParams) -> surf::Result<Vec<Category>> {
        let response = self
            .inner
            .get(&format!("categories?{}", params.to_query_string()))
            .recv_bytes()
            .await?;

        let response: DataResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#search-mods>
    pub async fn search(
        &self,
        params: &SearchParams,
    ) -> surf::Result<PaginatedDataResponse<Project>> {
        let request = self.inner.get("mods/search").query(params).unwrap().build();
        let mut response = self.inner.send(request).await?;

        let bytes = response.body_bytes().await?;

        let response = serde_json::from_slice(bytes.as_slice())?;

        Ok(response)
    }

    /// <https://docs.curseforge.com/#search-mods>
    ///
    /// This adheres to the limit of results defined by the
    /// [documentation](https://docs.curseforge.com/#pagination-limits),
    /// hardcoded by the constant [`API_PAGINATION_RESULTS_LIMIT`].
    pub async fn search_iter<'c>(
        &'c self,
        params: SearchParams,
    ) -> PaginatedStream<'_, SearchDelegate<'c>> {
        SearchDelegate::new(self, params).into()
    }

    /// <https://docs.curseforge.com/#get-mod>
    ///
    /// Renamed from `mod` to `addon` because the former is a keyword.
    pub async fn project(&self, mod_id: i32) -> surf::Result<Project> {
        let response = self
            .inner
            .get(&format!("mods/{}", mod_id))
            .recv_bytes()
            .await?;

        let response: DataResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-mods>
    pub async fn projects<I>(&self, mod_ids: I) -> surf::Result<Vec<Project>>
    where
        I: IntoIterator<Item = i32>,
    {
        let body = request_several_body!(mod_ids, i32, mod_ids.into_iter());
        let request = self.inner.post("mods").body_json(&body)?.build();

        let mut response = self.inner.send(request).await?;
        let bytes = response.body_bytes().await?;

        let response: DataResponse<_> = serde_json::from_slice(bytes.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-mod-file>
    pub async fn project_file(&self, mod_id: i32, file_id: i32) -> surf::Result<File> {
        let response = self
            .inner
            .get(&format!("mods/{}/files/{}", mod_id, file_id))
            .recv_bytes()
            .await?;

        let response: DataResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-files>
    pub async fn project_files(
        &self,
        mod_id: i32,
        params: Option<&ProjectFilesParams>,
    ) -> surf::Result<PaginatedDataResponse<File>> {
        let mut request = self.inner.post(&format!("mods/{}/files", mod_id));

        if let Some(params) = params {
            request = request.query(params)?;
        }

        let request = request.build();

        let mut response = self.inner.send(request).await?;
        let bytes = response.body_bytes().await?;

        let response = serde_json::from_slice(bytes.as_slice())?;

        Ok(response)
    }
}

pub struct SearchDelegate<'c> {
    client: &'c Client,
    params: SearchParams,
    pagination: Option<Pagination>,
}

impl<'c> SearchDelegate<'c> {
    pub fn new(client: &'c Client, mut params: SearchParams) -> Self {
        params.index = params.index.or(Some(0));

        Self {
            client,
            params,
            pagination: None,
        }
    }
}

#[async_trait]
impl PaginationDelegate for SearchDelegate<'_> {
    type Item = Project;
    type Error = surf::Error;

    async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error> {
        let result = self.client.search(&self.params).await;

        result.map(|response| {
            self.pagination = Some(response.pagination);
            response.data
        })
    }

    fn offset(&self) -> usize {
        self.params.index.unwrap() as usize
    }

    fn set_offset(&mut self, value: usize) {
        self.params.index = Some(value as i32);
    }

    fn total_items(&self) -> Option<usize> {
        self.pagination.as_ref().map(|pagination| {
            usize::min(
                API_PAGINATION_RESULTS_LIMIT,
                pagination.total_count as usize,
            )
        })
    }
}
