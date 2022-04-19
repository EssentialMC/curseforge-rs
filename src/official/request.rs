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
    pub struct ProjectSearchParams {
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

    impl ProjectSearchParams {
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
    #[serde(deny_unknown_fields)]
    pub struct DataResponse<T> {
        pub data: T,
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
    #[serde(deny_unknown_fields)]
    pub struct PaginatedDataResponse<T> {
        pub data: Vec<T>,
        pub pagination: Pagination,
    }
}

pub(crate) mod pagination {
    use async_trait::async_trait;
    use awaur::paginator::{PaginatedStream, PaginationDelegate};

    use super::params::{GamesParams, ProjectFilesParams, ProjectSearchParams};
    use crate::official::client::{Client, API_PAGINATION_RESULTS_LIMIT};
    use crate::official::types::{Game, Pagination, Project, ProjectFile};

    macro_rules! pagination_delegate {
        (
            $name:ident {
                item: $item:ty,
                pager: $pager:ident,
                $(vars: [$($var:ident: $var_type:ty),+],)?
                params: $params:ty,
            }
        ) => {
            /// See the documentation for [`PaginationDelegate`].
            pub struct $name<'c> {
                client: &'c Client,
                $($($var: $var_type,)*)?
                params: $params,
                pagination: Option<Pagination>,
            }

            impl<'c> $name<'c> {
                pub fn new(
                    client: &'c Client,
                    $($($var: $var_type,)*)?
                    mut params: $params,
                ) -> Self {
                    params.index = params.index.or(Some(0));

                    Self {
                        client,
                        $($($var,)*)?
                        params,
                        pagination: None,
                    }
                }
            }

            #[async_trait]
            impl PaginationDelegate for $name<'_> {
                type Item = $item;
                type Error = surf::Error;

                async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error> {
                    let result = self.client.$pager($($(self.$var,)*)? &self.params).await;

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
        };
    }

    pagination_delegate! {
        GamesDelegate {
            item: Game,
            pager: games,
            params: GamesParams,
        }
    }

    pagination_delegate! {
        ProjectSearchDelegate {
            item: Project,
            pager: search_projects,
            params: ProjectSearchParams,
        }
    }

    pagination_delegate! {
        ProjectFilesDelegate {
            item: ProjectFile,
            pager: project_files,
            vars: [project_id: i32],
            params: ProjectFilesParams,
        }
    }

    /// See the documentation for [`PaginatedStream`].
    pub type GamesStream<'c, 'f> = PaginatedStream<'f, GamesDelegate<'c>>;
    /// See the documentation for [`PaginatedStream`].
    pub type ProjectSearchStream<'c, 'f> = PaginatedStream<'f, ProjectSearchDelegate<'c>>;
    /// See the documentation for [`PaginatedStream`].
    pub type ProjectFilesStream<'c, 'f> = PaginatedStream<'f, ProjectFilesDelegate<'c>>;
}
