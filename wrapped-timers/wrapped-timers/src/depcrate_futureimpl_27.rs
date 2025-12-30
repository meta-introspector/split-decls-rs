// Generated macro for impl_27 (impl)
macro_rules! Depcrate_futureimpl_27 {
() => {
// Module: crate::future
// Provides: {"impl_27"}
// Dependencies: {}
impl IntervalStream { # [doc = " Create a new interval stream."] # [doc = ""] # [doc = " Remember that streams do nothing unless polled or spawned, so either"] # [doc = " spawn this stream via `wasm_bindgen_futures::spawn_local` or use it inside"] # [doc = " another stream or future."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " use futures_util::stream::StreamExt;"] # [doc = " use gloo_timers::future::IntervalStream;"] # [doc = " use wasm_bindgen_futures::spawn_local;"] # [doc = ""] # [doc = " spawn_local(async {"] # [doc = "     IntervalStream::new(1_000).for_each(|_| {"] # [doc = "         // Do stuff every one second..."] # [doc = "     }).await;"] # [doc = " });"] # [doc = " ```"] pub fn new (millis : u32) -> IntervalStream { let (sender , receiver) = mpsc :: unbounded () ; let inner = Interval :: new (millis , move | | { sender . unbounded_send (()) . unwrap_throw () ; }) ; IntervalStream { receiver , _inner : inner , } } }
};
}
