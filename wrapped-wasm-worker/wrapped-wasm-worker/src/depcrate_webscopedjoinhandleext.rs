// Generated macro for ScopedJoinHandleExt (trait)
macro_rules! Depcrate_webScopedJoinHandleExt {
() => {
// Module: crate::web
// Provides: {"ScopedJoinHandleExt"}
// Dependencies: {}
# [doc = " Web-specific extension for"] # [doc = " [`web_thread::ScopedJoinHandle`](crate::ScopedJoinHandle)."] pub trait ScopedJoinHandleExt < 'scope , T > { # [doc = " Async version of [`ScopedJoinHandle::join()`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " - If called on the thread to join."] # [doc = " - If it was already polled to completion by another call to"] # [doc = "   [`ScopedJoinHandleExt::join_async()`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(all(target_feature = \"atomics\", not(unsupported_spawn)))]"] # [doc = " # wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);"] # [doc = " # #[cfg_attr(all(target_feature = \"atomics\", not(unsupported_spawn)), wasm_bindgen_test::wasm_bindgen_test)]"] # [doc = " # async fn test() {"] # [doc = " use web_thread::web::{self, ScopedJoinHandleExt};"] # [doc = ""] # [doc = " web::scope_async(|scope| async {"] # [doc = " \tscope.spawn(|| ()).join_async().await.unwrap();"] # [doc = " }).await;"] # [doc = " # }"] # [doc = " # #[cfg(not(all(target_feature = \"atomics\", not(unsupported_spawn))))]"] # [doc = " # let _ = test();"] # [doc = " ```"] fn join_async < 'handle > (& 'handle mut self) -> ScopedJoinHandleFuture < 'handle , 'scope , T > ; }
};
}
