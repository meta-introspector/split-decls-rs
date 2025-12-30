// Generated macro for yield_now_async (function)
macro_rules! Depcrate_webyield_now_async {
() => {
// Module: crate::web
// Provides: {"yield_now_async"}
// Dependencies: {}
# [doc = " Async version of [`yield_now()`](std::thread::yield_now). This yields"] # [doc = " execution to the [event loop]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " This is no-op in worklets."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);"] # [doc = " # #[wasm_bindgen_test::wasm_bindgen_test]"] # [doc = " # async fn test() {"] # [doc = " use web_thread::web::{self, YieldTime};"] # [doc = ""] # [doc = " # fn long_running_task() -> bool { false }"] # [doc = " while long_running_task() {"] # [doc = " \tweb::yield_now_async(YieldTime::default()).await"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [event loop]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Event_loop"] pub fn yield_now_async (time : YieldTime) -> YieldNowFuture { YieldNowFuture (thread :: YieldNowFuture :: new (time)) }
};
}
