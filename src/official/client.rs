use std::collections::VecDeque;

use async_stream::try_stream;
use futures_core::Stream;

use super::types::*;

/// This is the official CurseForge Core API base URL.
/// You must pass it to constructors explicitly.
pub const DEFAULT_API_BASE: &str = "https://api.curseforge.com/v1/";

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

    pub async fn search_mods_iter(
        &self,
        mut params: SearchModsParams,
    ) -> impl Stream<Item = surf::Result<Mod>> + '_ {
        let mut items = VecDeque::new();
        params.index = params.index.or(Some(0));

        try_stream! {
            let mut response = self.search_mods(&params).await?;

            loop {
                if items.is_empty() {
                    if params.index.unwrap() as i64 >= response.pagination.total_count {
                        break;
                    }

                    response = self.search_mods(&params).await?;
                    debug_assert_eq!(response.pagination.index, params.index.unwrap());
                    params.index = Some(params.index.unwrap() + response.pagination.result_count);
                    debug_assert_eq!(response.pagination.result_count as usize, response.data.len());

                    items.extend(response.data.into_iter());
                }

                yield items.pop_front().unwrap();
            }
        }
    }
}
