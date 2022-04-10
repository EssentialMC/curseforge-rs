#[doc(inline)]
pub use paginated::*;
#[doc(inline)]
pub use params::*;
#[doc(inline)]
pub use response::*;

pub mod params {
    use query_string::QueryString;
    use serde::Serialize;
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use serde_with::{DeserializeFromStr, SerializeDisplay};
    use strum::{Display, EnumString};

    use crate::official::types::ModLoaderType;

    /// <https://docs.curseforge.com/#get-games>
    #[derive(Clone, Debug, Default, PartialEq, Serialize, QueryString)]
    #[serde(rename_all = "camelCase")]
    pub struct GamesParams {
        pub index: Option<i32>,
        pub page_size: Option<i32>,
    }

    /// <https://docs.curseforge.com/#get-categories>
    #[derive(Clone, Debug, PartialEq, Serialize, QueryString)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, QueryString)]
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
}

pub(crate) mod body {
    macro_rules! request_several_body {
        ($field:ident, $field_type:ty, $iter:expr) => {{
            use ::serde::Serialize;

            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct _RequestBody {
                $field: Vec<$field_type>,
            }

            _RequestBody {
                $field: $iter.collect(),
            }
        }};
    }

    pub(crate) use request_several_body;
}

pub mod response {
    use crate::official::types::Pagination;
    use serde::{Deserialize, Serialize};

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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct DataResponse<T> {
        pub data: T,
    }

    /// Wraps API responses which have the fields `data` and `pagination`.
    ///
    /// - <https://docs.curseforge.com/#tocS_Get%20Games%20Response>
    /// - <https://docs.curseforge.com/#tocS_Search%20Mods%20Response>
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PaginatedDataResponse<T> {
        pub data: Vec<T>,
        pub pagination: Pagination,
    }
}

pub mod paginated {
    use std::cell::RefCell;
    use std::pin::Pin;
    use std::rc::Rc;
    use std::task::{Context, Poll};

    use futures_lite::stream::{Stream, StreamExt};

    use crate::official::types::core::Pagination;

    /// This is a convenience wrapper provided to give the callee access to the
    /// API's limit of maximum results, as well as the [`Pagination`] resulting
    /// from every new request. The heavy lifting here is done by the
    /// closure, which must return the [`Stream`] that this is intended to
    /// wrap. For an example of such an implementation, see the source for
    /// the [`crate::official::client::Client::search_iter`] method.
    pub struct PaginatedStream<'ps, T> {
        // This needs to be pinned because `Stream` should be immovable,
        // and stored on the heap because dynamic-dispatch values are dynamically sized.
        inner: Pin<Box<dyn Stream<Item = surf::Result<T>> + 'ps>>,
        // Store the last `Pagination` from the API, as a reference-counted
        // reference cell, so that the closure that implements the `Stream`
        // can update its value.
        pagination: Rc<RefCell<Option<Pagination>>>,
        // This will be assigned by the constructor, causing the stream to yield
        // `Poll::Ready(None)` when we have reached the API's result limit.
        // If there is no known limit, `usize::MAX` can be used.
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
            // Create a new `Option<Pagination>` with interior mutability to pass to the
            // `Stream` closure. This will be `None` until the first request
            // succeeds.
            let pagination = Rc::new(RefCell::new(None));

            Self {
                // Clone the reference-counted pointer to the `Pagination`,
                // and provide the API's `limit` to the closure, so it knows when to stop.
                inner: stream(Rc::clone(&pagination), limit),
                // Store the original `Rc` to the state of the paginator.
                pagination,
                // Store the limit for maximum results. A callee can get the value of this
                // with the `limit()` method.
                limit,
            }
        }

        pub fn limit(&self) -> usize {
            self.limit
        }

        pub fn pagination(&self) -> Option<Pagination> {
            // Get the `RefMut`, and then a `Ref`, and clone the `Pagination`
            // so that we can pass ownership of a new value.
            (*self.pagination.as_ref().borrow()).clone()
        }
    }

    impl<T> Stream for PaginatedStream<'_, T> {
        type Item = surf::Result<T>;

        fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.inner.poll_next(ctx)
        }

        /// Return the lower and upper bounds for the stream, the upper bound
        /// will be `None` if no successful requests have been made yet.
        /// This does not provide the total number of results returned
        /// from the API, but rather is capped at the limit provided to
        /// the constructor for this paginator. The lower bound is
        /// always going to be zero.
        fn size_hint(&self) -> (usize, Option<usize>) {
            // Get the `RefMut`, and then a `Ref`, dereference to get a borrow.
            match *self.pagination.as_ref().borrow() {
                Some(Pagination { total_count, .. }) => {
                    (0, Some(usize::min(self.limit, total_count as usize)))
                }
                None => (0, None),
            }
        }
    }
}
