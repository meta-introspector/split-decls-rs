use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> ChromeLayerBuilder<S>
where
    S: Subscriber + for<'span> LookupSpan<'span> + Send + Sync,
{
    pub fn new() -> Self {
        ChromeLayerBuilder {
            out_writer: None,
            name_fn: None,
            cat_fn: None,
            include_args: false,
            include_locations: true,
            trace_style: TraceStyle::Threaded,
            _inner: PhantomData,
        }
    }
    /// Set the file to which to output the trace.
    ///
    /// Defaults to `./trace-{unix epoch in micros}.json`.
    ///
    /// # Panics
    ///
    /// If `file` could not be opened/created. To handle errors,
    /// open a file and pass it to [`writer`](crate::ChromeLayerBuilder::writer) instead.
    pub fn file<P: AsRef<Path>>(self, file: P) -> Self {
        self.writer(std::fs::File::create(file).expect("Failed to create trace file."))
    }
    /// Supply an arbitrary writer to which to write trace contents.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tracing_chrome::ChromeLayerBuilder;
    /// # use tracing_subscriber::prelude::*;
    /// let (layer, guard) = ChromeLayerBuilder::new().writer(std::io::sink()).build();
    /// # tracing_subscriber::registry().with(layer).init();
    /// ```
    pub fn writer<W: Write + Send + 'static>(mut self, writer: W) -> Self {
        self.out_writer = Some(Box::new(writer));
        self
    }
    /// Include arguments in each trace entry.
    ///
    /// Defaults to `false`.
    ///
    /// Includes the arguments used when creating a span/event
    /// in the "args" section of the trace entry.
    pub fn include_args(mut self, include: bool) -> Self {
        self.include_args = include;
        self
    }
    /// Include file+line with each trace entry.
    ///
    /// Defaults to `true`.
    ///
    /// This can add quite a bit of data to the output so turning
    /// it off might be helpful when collecting larger traces.
    pub fn include_locations(mut self, include: bool) -> Self {
        self.include_locations = include;
        self
    }
    /// Sets the style used when recording trace events.
    ///
    /// See [`TraceStyle`](crate::TraceStyle) for details.
    pub fn trace_style(mut self, style: TraceStyle) -> Self {
        self.trace_style = style;
        self
    }
    /// Allows supplying a function that derives a name from
    /// an Event or Span. The result is used as the "name" field
    /// on trace entries.
    ///
    /// # Example
    /// ```
    /// use tracing_chrome::{ChromeLayerBuilder, EventOrSpan};
    /// use tracing_subscriber::{registry::Registry, prelude::*};
    ///
    /// let (chrome_layer, _guard) = ChromeLayerBuilder::new().name_fn(Box::new(|event_or_span| {
    ///     match event_or_span {
    ///         EventOrSpan::Event(ev) => { ev.metadata().name().into() },
    ///         EventOrSpan::Span(_s) => { "span".into() },
    ///     }
    /// })).build();
    /// tracing_subscriber::registry().with(chrome_layer).init()
    /// ```
    pub fn name_fn(mut self, name_fn: NameFn<S>) -> Self {
        self.name_fn = Some(name_fn);
        self
    }
    /// Allows supplying a function that derives a category from
    /// an Event or Span. The result is used as the "cat" field on
    /// trace entries.
    ///
    /// # Example
    /// ```
    /// use tracing_chrome::{ChromeLayerBuilder, EventOrSpan};
    /// use tracing_subscriber::{registry::Registry, prelude::*};
    ///
    /// let (chrome_layer, _guard) = ChromeLayerBuilder::new().category_fn(Box::new(|_| {
    ///     "my_module".into()
    /// })).build();
    /// tracing_subscriber::registry().with(chrome_layer).init()
    /// ```
    pub fn category_fn(mut self, cat_fn: NameFn<S>) -> Self {
        self.cat_fn = Some(cat_fn);
        self
    }
    /// Creates a [`ChromeLayer`](crate::ChromeLayer) and associated [`FlushGuard`](crate::FlushGuard).
    ///
    /// # Panics
    ///
    /// If no file or writer was specified and the default trace file could not be opened/created.
    pub fn build(self) -> (ChromeLayer<S>, FlushGuard) {
        ChromeLayer::new(self)
    }
}
