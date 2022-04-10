use std::collections::VecDeque;

use async_stream::try_stream;

use super::paginate::*;
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

    /// <https://docs.curseforge.com/#get-game>
    pub async fn game(&self, game_id: i32) -> surf::Result<Game> {
        let response = self
            .inner
            .get(&format!("games/{}", game_id))
            .recv_bytes()
            .await?;

        let response: IntermResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-games>
    pub async fn games(&self, params: &GamesParams) -> surf::Result<GamesResponse> {
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

        let response: IntermResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-version-types>
    pub async fn game_version_types(&self, game_id: i32) -> surf::Result<Vec<GameVersionType>> {
        let response = self
            .inner
            .get(&format!("games/{}/version-types", game_id))
            .recv_bytes()
            .await?;

        let response: IntermResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#get-categories>
    pub async fn categories(&self, params: &CategoriesParams) -> surf::Result<Vec<Category>> {
        let response = self
            .inner
            .get(&format!("categories?{}", params.to_query_string()))
            .recv_bytes()
            .await?;

        let response: IntermResponse<_> = serde_json::from_slice(response.as_slice())?;

        Ok(response.data)
    }

    /// <https://docs.curseforge.com/#search-mods>
    pub async fn search(&self, params: &SearchParams) -> surf::Result<SearchProjectsResponse> {
        let response = self
            .inner
            .get(&format!("mods/search?{}", params.to_query_string()))
            .recv_bytes()
            .await?;

        let response = serde_json::from_slice(response.as_slice())?;

        Ok(response)
    }

    /// <https://docs.curseforge.com/#search-mods>
    ///
    /// This adheres to the limit of results defined by the
    /// [documentation](https://docs.curseforge.com/#pagination-limits),
    /// hardcoded by the constant [`API_PAGINATION_RESULTS_LIMIT`].
    pub async fn search_iter(&self, mut params: SearchParams) -> PaginatedStream<'_, Project> {
        PaginatedStream::new(
            |pagination, limit| {
                // Construct a new iterator that can have items popped from the front.
                let mut items = VecDeque::new();
                // If the callee didn't specify a starting index, set it to `0` so that
                // this value can be conveniently unwrapped.
                params.index = params.index.or(Some(0));

                Box::pin(try_stream! {
                    // Initialize `response` with the first result.
                    let mut response = self.search_projects(&params).await?;

                    // Assign the response's `Pagination` value
                    // into the `RefCell` provided in the arguments.
                    {
                        let mut pagination = pagination.as_ref().borrow_mut();
                        *pagination = Some(response.pagination);
                    }

                    loop {
                        if items.is_empty() {
                            // Get a mutable reference to the current `Pagination`.
                            // A `RefMut` is taken, but will not be mutated unless
                            // we are still under the limit below.
                            let mut pagination = pagination.as_ref().borrow_mut();
                            // The limit that we will use to break the iterator is
                            // going to be either the maximum results the API will
                            // provide before terminating the stream,
                            // or the current `total_count` provided by the last request.
                            let limit = usize::min(
                                limit,
                                pagination.as_ref().unwrap().total_count as usize
                            );

                            // Check if we are at or past the limit above and break,
                            // resulting in a `Poll::Ready(None)`.
                            if params.index.unwrap() as usize >= limit {
                                break;
                            }

                            // This has continued, meaning we haven't yet reached the limit.
                            // Get the next page of search results, the index for which
                            // has been set on the previous iteration (or before entering the loop).
                            response = self.search_projects(&params).await?;
                            // Assign the response's new `Pagination` to the `RefMut`
                            // from the arguments, this will be available by
                            // the `pagination()` method on `PaginatedStream`.
                            *pagination = Some(response.pagination);
                            // Get the current `Option<Pagination>` as a reference to the inner
                            // value and unwrap the `Option`.
                            let pagination = (*pagination).as_ref().unwrap();

                            // Check that the current `pagination`'s index matches the
                            // one from the `SearchModsParams` to ensure it is being updated correctly.
                            debug_assert_eq!(pagination.index, params.index.unwrap());
                            // Check that the API is returning the number of results that it
                            // claims to be providing.
                            debug_assert_eq!(pagination.result_count as usize, response.data.len());

                            // Assign the proper offset for the next page request.
                            params.index = Some(params.index.unwrap() + pagination.result_count);
                            // Take the `Vec<Mod>` items from the response and extend the
                            // `VecDeque` that is used to yield the front-most items.
                            items.extend(response.data.into_iter());
                        }

                        // Take the front-most item from the state's `VecDeque` and unwrap it,
                        // because we already checked to ensure that it isn't empty.
                        yield items.pop_front().unwrap();
                    }
                })
            },
            API_PAGINATION_RESULTS_LIMIT,
        )
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

        let response: IntermResponse<_> = serde_json::from_slice(response.as_slice())?;

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

        let response: IntermResponse<_> = serde_json::from_slice(bytes.as_slice())?;

        Ok(response.data)
    }
}
