use std::cell::RefCell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use async_stream::try_stream;
use futures_lite::stream::{Stream, StreamExt};

use super::types::*;

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

    /// <https://docs.curseforge.com/#get-games>
    pub async fn games(&self, params: &GamesParams) -> surf::Result<GamesResponse> {
        Ok(self
            .inner
            .get(&format!("games?{}", params.to_query_string()))
            .recv_json()
            .await?)
    }

    /// <https://docs.curseforge.com/#get-game>
    pub async fn game(&self, game_id: i32) -> surf::Result<Game> {
        Ok(self
            .inner
            .get(&format!("games/{}", game_id))
            .recv_json::<GameResponse>()
            .await?
            .data)
    }

    /// <https://docs.curseforge.com/#get-versions>
    pub async fn game_versions(&self, game_id: i32) -> surf::Result<Vec<GameVersions>> {
        Ok(self
            .inner
            .get(&format!("games/{}/versions", game_id))
            .recv_json::<GameVersionsResponse>()
            .await?
            .data)
    }

    /// <https://docs.curseforge.com/#get-version-types>
    pub async fn game_version_types(&self, game_id: i32) -> surf::Result<Vec<GameVersionType>> {
        Ok(self
            .inner
            .get(&format!("games/{}/version-types", game_id))
            .recv_json::<GameVersionTypesResponse>()
            .await?
            .data)
    }

    /// <https://docs.curseforge.com/#get-categories>
    pub async fn categories(&self, params: &CategoriesParams) -> surf::Result<Vec<Category>> {
        Ok(self
            .inner
            .get(&format!("categories?{}", params.to_query_string()))
            .recv_json::<CategoriesResponse>()
            .await?
            .data)
    }

    /// <https://docs.curseforge.com/#search-mods>
    pub async fn search_mods(&self, params: &SearchModsParams) -> surf::Result<SearchModsResponse> {
        let response = self
            .inner
            .get(&format!("mods/search?{}", params.to_query_string()))
            .recv_bytes()
            .await?;

        std::fs::write("./search.json", &response).unwrap();

        Ok(serde_json::from_slice(response.as_slice())?)
    }

    /// <https://docs.curseforge.com/#search-mods>
    pub async fn search_mods_iter(&self, mut params: SearchModsParams) -> PaginatedStream<'_, Mod> {
        PaginatedStream::new(
            |pagination, limit| {
                let mut items = VecDeque::new();
                params.index = params.index.or(Some(0));

                Box::pin(try_stream! {
                    let mut response = self.search_mods(&params).await?;

                    {
                        let mut pagination = pagination.as_ref().borrow_mut();
                        *pagination = Some(response.pagination);
                    }

                    loop {
                        if items.is_empty() {
                            let mut pagination = pagination.as_ref().borrow_mut();

                            let limit = usize::min(
                                limit,
                                pagination.as_ref().unwrap().total_count as usize
                            );

                            if params.index.unwrap() as usize >= limit {
                                break;
                            }

                            response = self.search_mods(&params).await?;
                            *pagination = Some(response.pagination);
                            let pagination = (*pagination).as_ref().unwrap();

                            debug_assert_eq!(pagination.index, params.index.unwrap());
                            debug_assert_eq!(pagination.result_count as usize, response.data.len());

                            params.index = Some(params.index.unwrap() + pagination.result_count);

                            items.extend(response.data.into_iter());
                        }

                        yield items.pop_front().unwrap();
                    }
                })
            },
            API_PAGINATION_RESULTS_LIMIT,
        )
    }
}

pub struct PaginatedStream<'ps, T> {
    inner: Pin<Box<dyn Stream<Item = surf::Result<T>> + 'ps>>,
    pagination: Rc<RefCell<Option<Pagination>>>,
    limit: usize,
}

impl<'ps, T> PaginatedStream<'ps, T> {
    pub fn new<F>(stream: F, limit: usize) -> Self
    where
        F: FnOnce(
            Rc<RefCell<Option<Pagination>>>,
            usize,
        ) -> Pin<Box<dyn Stream<Item = surf::Result<T>> + 'ps>>,
    {
        let pagination = Rc::new(RefCell::new(None));

        Self {
            inner: stream(Rc::clone(&pagination), limit),
            pagination,
            limit,
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn pagination(&self) -> Option<Pagination> {
        (*self.pagination.as_ref().borrow()).clone()
    }
}

impl<T> Stream for PaginatedStream<'_, T> {
    type Item = surf::Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_next(ctx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match *self.pagination.as_ref().borrow() {
            Some(Pagination { total_count, .. }) => {
                (0, Some(usize::min(self.limit, total_count as usize)))
            }
            None => (0, None),
        }
    }
}
