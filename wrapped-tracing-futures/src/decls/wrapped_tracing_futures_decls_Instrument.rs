use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extension trait allowing futures, streams, sinks, and executors to be
/// instrumented with a `tracing` [span].
///
/// [span]: mod@tracing::span
pub trait Instrument: Sized {
    /// Instruments this type with the provided [`Span`], returning an
    /// [`Instrumented`] wrapper.
    ///
    /// If the instrumented type is a future, stream, or sink, the attached
    /// [`Span`] will be [entered] every time it is polled or [`Drop`]ped. If
    /// the instrumented type is a future executor, every future spawned on that
    /// executor will be instrumented by the attached [`Span`].
    ///
    /// # Examples
    ///
    /// Instrumenting a future:
    ///
    /// ```rust,ignore
    /// use tracing_futures::Instrument;
    ///
    /// # async fn doc() {
    /// let my_future = async {
    ///     // ...
    /// };
    ///
    /// my_future
    ///     .instrument(tracing::info_span!("my_future"))
    ///     .await
    /// # }
    /// ```
    ///
    /// [entered]: Span::enter()
    fn instrument(self, span: Span) -> Instrumented<Self> {
        #[cfg(feature = "std-future")]
        let inner = ManuallyDrop::new(self);
        #[cfg(not(feature = "std-future"))]
        let inner = self;
        Instrumented { inner, span }
    }
    /// Instruments this type with the [current] [`Span`], returning an
    /// [`Instrumented`] wrapper.
    ///
    /// If the instrumented type is a future, stream, or sink, the attached
    /// [`Span`] will be [entered] every time it is polled or [`Drop`]ped. If
    /// the instrumented type is a future executor, every future spawned on that
    /// executor will be instrumented by the attached [`Span`].
    ///
    /// This can be used to propagate the current span when spawning a new future.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use tracing_futures::Instrument;
    ///
    /// # async fn doc() {
    /// let span = tracing::info_span!("my_span");
    /// let _enter = span.enter();
    ///
    /// // ...
    ///
    /// let future = async {
    ///     tracing::debug!("this event will occur inside `my_span`");
    ///     // ...
    /// };
    /// tokio::spawn(future.in_current_span());
    /// # }
    /// ```
    ///
    /// [current]: Span::current()
    /// [entered]: Span::enter()
    #[inline]
    fn in_current_span(self) -> Instrumented<Self> {
        self.instrument(Span::current())
    }
}
