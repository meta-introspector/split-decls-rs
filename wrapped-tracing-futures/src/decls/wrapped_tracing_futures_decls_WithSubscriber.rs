use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extension trait allowing futures, streams, and sinks to be instrumented with
/// a `tracing` [`Subscriber`].
///
/// [`Subscriber`]: tracing::Subscriber
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub trait WithSubscriber: Sized {
    /// Attaches the provided [`Subscriber`] to this type, returning a
    /// `WithDispatch` wrapper.
    ///
    /// When the wrapped type is a future, stream, or sink, the attached
    /// subscriber will be set as the [default] while it is being polled.
    /// When the wrapped type is an executor, the subscriber will be set as the
    /// default for any futures spawned on that executor.
    ///
    /// [`Subscriber`]: tracing::Subscriber
    /// [default]: tracing::dispatcher#setting-the-default-subscriber
    fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>
    where
        S: Into<Dispatch>,
    {
        WithDispatch {
            inner: self,
            dispatch: subscriber.into(),
        }
    }
    /// Attaches the current [default] [`Subscriber`] to this type, returning a
    /// `WithDispatch` wrapper.
    ///
    /// When the wrapped type is a future, stream, or sink, the attached
    /// subscriber will be set as the [default] while it is being polled.
    /// When the wrapped type is an executor, the subscriber will be set as the
    /// default for any futures spawned on that executor.
    ///
    /// This can be used to propagate the current dispatcher context when
    /// spawning a new future.
    ///
    /// [`Subscriber`]: tracing::Subscriber
    /// [default]: tracing::dispatcher#setting-the-default-subscriber
    #[inline]
    fn with_current_subscriber(self) -> WithDispatch<Self> {
        WithDispatch {
            inner: self,
            dispatch: dispatcher::get_default(|default| default.clone()),
        }
    }
}
