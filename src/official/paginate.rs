use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use futures_lite::stream::{Stream, StreamExt};

use super::types::core::Pagination;

/// This is a convenience wrapper provided to give the callee access to the
/// API's limit of maximum results, as well as the [`Pagination`] resulting from
/// every new request. The heavy lifting here is done by the closure, which must
/// return the [`Stream`] that this is intended to wrap. For an example of such
/// an implementation, see the source for the [`Client::search_mods_iter`]
/// method.
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
            // and provide the API's `limit` to the closure so it knows when to stop.
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

    /// Return the lower and upper bounds for the stream, the upper bound will
    /// be `None` if no successful requests have been made yet. This does not
    /// provide the total number of results returned from the API, but rather is
    /// capped at the limit provided to the constructor for this paginator.
    /// The lower bound is always going to be zero.
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
