use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A [`Layer`](tracing_subscriber::Layer) that writes a Chrome trace file.
pub struct ChromeLayer<S>
where
    S: Subscriber + for<'span> LookupSpan<'span> + Send + Sync,
{
    out: Arc<Mutex<Sender<Message>>>,
    start: std::time::Instant,
    max_tid: AtomicUsize,
    include_args: bool,
    include_locations: bool,
    trace_style: TraceStyle,
    name_fn: Option<NameFn<S>>,
    cat_fn: Option<NameFn<S>>,
    _inner: PhantomData<S>,
}
