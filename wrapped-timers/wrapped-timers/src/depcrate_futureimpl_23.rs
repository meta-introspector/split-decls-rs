// Generated macro for impl_23 (impl)
macro_rules! Depcrate_futureimpl_23 {
() => {
// Module: crate::future
// Provides: {"impl_23"}
// Dependencies: {}
impl TimeoutFuture { # [doc = " Create a new timeout future."] # [doc = ""] # [doc = " Remember that futures do nothing unless polled or spawned, so either"] # [doc = " pass this future to `wasm_bindgen_futures::spawn_local` or use it inside"] # [doc = " another future."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use gloo_timers::future::TimeoutFuture;"] # [doc = " use wasm_bindgen_futures::spawn_local;"] # [doc = ""] # [doc = " spawn_local(async {"] # [doc = "     TimeoutFuture::new(1_000).await;"] # [doc = "     // Do stuff after one second..."] # [doc = " });"] # [doc = " ```"] pub fn new (millis : u32) -> TimeoutFuture { let (tx , rx) = oneshot :: channel () ; let inner = Timeout :: new (millis , move | | { tx . send (()) . unwrap_throw () ; }) ; TimeoutFuture { _inner : inner , rx } } }
};
}
