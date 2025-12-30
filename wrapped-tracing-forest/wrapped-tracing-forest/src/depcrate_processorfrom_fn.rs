// Generated macro for from_fn (function)
macro_rules! Depcrate_processorfrom_fn {
() => {
// Module: crate::processor
// Provides: {"from_fn"}
// Dependencies: {}
# [doc = " Create a processor that processes incoming logs via a function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Internally, [`worker_task`] uses `from_fn` to allow the subscriber to send"] # [doc = " trace data across a channel to a processing task."] # [doc = " ```"] # [doc = " use tokio::sync::mpsc;"] # [doc = " use tracing_forest::processor;"] # [doc = ""] # [doc = " let (tx, rx) = mpsc::unbounded_channel();"] # [doc = ""] # [doc = " let sender_processor = processor::from_fn(move |tree| tx"] # [doc = "     .send(tree)"] # [doc = "     .map_err(|err| {"] # [doc = "         let msg = err.to_string().into();"] # [doc = "         processor::error(err.0, msg)"] # [doc = "     })"] # [doc = " );"] # [doc = ""] # [doc = " // -- snip --"] # [doc = " ```"] # [doc = ""] # [doc = " [`worker_task`]: crate::runtime::worker_task"] pub fn from_fn < F > (f : F) -> FromFn < F > where F : 'static + Fn (Tree) -> Result , { FromFn (f) }
};
}
