pub use pagination::*;
pub use params::*;
pub use response::*;

pub(crate) mod params {
    use serde::Serialize;
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use serde_with::{DeserializeFromStr, SerializeDisplay};
    use strum::{Display, EnumString};

    use crate::official::types::ModLoaderType;

    /// <https://docs.curseforge.com/#get-games>
    #[derive(Clone, Debug, Default, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GamesParams {
        pub index: Option<i32>,
        pub page_size: Option<i32>,
    }

    /// <https://docs.curseforge.com/#get-categories>
    #[derive(Clone, Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CategoriesParams {
        pub game_id: i32,
        pub class_id: Option<i32>,
    }

    impl CategoriesParams {
        pub fn game(game_id: i32) -> Self {
            Self {
                game_id,
                class_id: None,
            }
        }
    }

    /// <https://docs.curseforge.com/#search-mods>
    #[derive(Clone, Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SearchParams {
        pub game_id: i32,
        pub class_id: Option<i32>,
        pub category_id: Option<i32>,
        pub game_version: Option<String>,
        pub search_filter: Option<String>,
        pub sort_field: Option<SearchSort>,
        pub sort_order: Option<SearchSortOrder>,
        pub mod_loader_type: Option<ModLoaderType>,
        pub game_version_type_id: Option<i32>,
        pub slug: Option<String>,
        pub index: Option<i32>,
        pub page_size: Option<i32>,
    }

    impl SearchParams {
        pub fn game(game_id: i32) -> Self {
            Self {
                game_id,
                class_id: None,
                category_id: None,
                game_version: None,
                search_filter: None,
                sort_field: None,
                sort_order: None,
                mod_loader_type: None,
                game_version_type_id: None,
                slug: None,
                index: None,
                page_size: None,
            }
        }
    }

    /// <https://docs.curseforge.com/#tocS_ModsSearchSortField>
    #[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum SearchSort {
        Featured = 1,
        Popularity = 2,
        LastUpdated = 3,
        Name = 4,
        Author = 5,
        TotalDownloads = 6,
        Category = 7,
        GameVersion = 8,
    }

    /// <https://docs.curseforge.com/#tocS_SortOrder>
    #[derive(
        Clone, Debug, PartialEq, EnumString, Display, SerializeDisplay, DeserializeFromStr,
    )]
    pub enum SearchSortOrder {
        #[strum(serialize = "asc")]
        Ascending,
        #[strum(serialize = "desc")]
        Descending,
    }

    /// <https://docs.curseforge.com/#get-mod-files>
    #[derive(Clone, Debug, Default, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectFilesParams {
        pub game_version: Option<String>,
        pub mod_loader_type: Option<ModLoaderType>,
        pub game_version_type_id: Option<i32>,
        pub index: Option<i32>,
        pub page_size: Option<i32>,
    }

    macro_rules! several_body {
        ($field:literal, $field_type:ty, $iter:expr) => {{
            use ::serde::Serialize;

            #[derive(Serialize)]
            struct __RequestBody {
                #[serde(rename = $field)]
                __field: Vec<$field_type>,
            }

            __RequestBody {
                __field: $iter.collect(),
            }
        }};
    }

    pub(crate) use several_body;

    /// <https://docs.curseforge.com/#tocS_GetFeaturedModsRequestBody>
    #[derive(Clone, Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FeaturedProjectsBody {
        pub game_id: i32,
        pub excluded_mod_ids: Vec<i32>,
        pub game_version_type_id: Option<i32>,
    }

    impl FeaturedProjectsBody {
        pub fn game(game_id: i32) -> Self {
            Self {
                game_id,
                excluded_mod_ids: Vec::new(),
                game_version_type_id: None,
            }
        }
    }
}

pub(crate) mod response {
    use serde::{Deserialize, Serialize};

    use crate::official::types::Pagination;

    /// Wraps API responses which have the single field `data`.
    /// Methods that make calls to endpoints that return this will unwrap it
    /// and provide the value of `data` directly.
    ///
    /// - <https://docs.curseforge.com/#tocS_Get%20Versions%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Version%20Types%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Categories%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Game%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Mod%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Mods%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Featured%20Mods%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Mod%20File%20Response>
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct DataResponse<T> {
        pub data: T,
    }

    /// Wraps API responses which have the fields `data` and `pagination`.
    ///
    /// - <https://docs.curseforge.com/#tocS_Get%20Games%20Response>
    /// - <https://docs.curseforge.com/#tocS_Search%20Mods%20Response>
    /// - <https://docs.curseforge.com/#tocS_Get%20Mod%20Files%20Response>
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PaginatedDataResponse<T> {
        pub data: Vec<T>,
        pub pagination: Pagination,
    }
}

pub(crate) mod pagination {
    use async_trait::async_trait;
    use awaur::paginator::PaginationDelegate;

    use super::params::{ProjectFilesParams, SearchParams};
    use crate::official::client::{Client, API_PAGINATION_RESULTS_LIMIT};
    use crate::official::types::{File, Pagination, Project};

    macro_rules! impl_pagination_delegate {
        (
            for $target:ty {
                $self:ident,
                item: $item:ty,
                pager: ($($pager_frag:tt)*),
            }
        ) => {
            #[async_trait]
            impl PaginationDelegate for $target {
                type Item = $item;
                type Error = surf::Error;

                async fn next_page(&mut $self) -> Result<Vec<Self::Item>, Self::Error> {
                    let result = $($pager_frag)*.await;

                    result.map(|response| {
                        $self.pagination = Some(response.pagination);
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
        };
    }

    /// See the documentation for [`PaginationDelegate`].
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

    impl_pagination_delegate! {
        for SearchDelegate<'_> {
            self,
            item: Project,
            pager: (self.client.search(&self.params)),
        }
    }

    /// See the documentation for [`PaginationDelegate`].
    pub struct ProjectFilesDelegate<'c> {
        client: &'c Client,
        project_id: i32,
        params: ProjectFilesParams,
        pagination: Option<Pagination>,
    }

    impl<'c> ProjectFilesDelegate<'c> {
        pub fn new(
            client: &'c Client,
            project_id: i32,
            params: Option<ProjectFilesParams>,
        ) -> Self {
            let mut params = params.unwrap_or_default();
            params.index = params.index.or(Some(0));

            Self {
                client,
                project_id,
                params,
                pagination: None,
            }
        }
    }

    impl_pagination_delegate! {
        for ProjectFilesDelegate<'_> {
            self,
            item: File,
            pager: (self.client.project_files(self.project_id, Some(&self.params))),
        }
    }
}
