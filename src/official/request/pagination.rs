use async_trait::async_trait;
use awaur::paginator::{PaginatedStream, PaginationDelegate};

use super::params::{GamesParams, ProjectFilesParams, ProjectSearchParams};
use crate::official::endpoints as e;
use crate::official::endpoints::API_PAGINATION_RESULTS_LIMIT;
use crate::official::types::{Game, Pagination, Project, ProjectFile};
use crate::Error;

macro_rules! pagination_delegate {
    (
        $name:ident {
            item: $item:ty,
            pager: $pager:path,
            $(vars: [$($var:ident: $var_type:ty),+],)?
            params: $params:ty,
        }
    ) => {
        /// See the documentation for [`PaginationDelegate`].
        pub struct $name<'cu> {
            client: &'cu isahc::HttpClient,
            base: &'cu url::Url,
            $($($var: $var_type,)*)?
            params: $params,
            pagination: Option<Pagination>,
        }

        impl<'cu> $name<'cu> {
            /// Constructs a new implementor of [`PaginationDelegate`]
            /// provided references to an [`isahc::HttpClient`] and a base URL.
            pub fn new(
                client: &'cu isahc::HttpClient,
                base: &'cu url::Url,
                $($($var: $var_type,)*)?
                mut params: $params,
            ) -> Self {
                params.index = params.index.or(Some(0));

                Self {
                    client,
                    base,
                    $($($var,)*)?
                    params,
                    pagination: None,
                }
            }
        }

        #[async_trait]
        impl PaginationDelegate for $name<'_> {
            type Item = $item;
            type Error = Error;

            async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error> {
                let result = $pager(
                        self.client,
                        self.base,
                        $($(self.$var,)*)?
                        &self.params
                    )
                    .await?
                    .into_value();
                self.pagination = Some(result.pagination);
                Ok(result.data)
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
        pager: e::games,
        params: GamesParams,
    }
}

pagination_delegate! {
    ProjectSearchDelegate {
        item: Project,
        pager: e::search_projects,
        params: ProjectSearchParams,
    }
}

pagination_delegate! {
    ProjectFilesDelegate {
        item: ProjectFile,
        pager: e::project_files,
        vars: [project_id: i32],
        params: ProjectFilesParams,
    }
}

/// See the documentation for [`PaginatedStream`].
pub type GamesStream<'cu, 'f> = PaginatedStream<'f, GamesDelegate<'cu>>;
/// See the documentation for [`PaginatedStream`].
pub type ProjectSearchStream<'cu, 'f> = PaginatedStream<'f, ProjectSearchDelegate<'cu>>;
/// See the documentation for [`PaginatedStream`].
pub type ProjectFilesStream<'cu, 'f> = PaginatedStream<'f, ProjectFilesDelegate<'cu>>;
