// Generated macro for JoinHandleExt (trait)
macro_rules! Depcrate_webJoinHandleExt {
() => {
// Module: crate::web
// Provides: {"JoinHandleExt"}
// Dependencies: {}
# [doc = " Web-specific extension for [`web_thread::JoinHandle`](crate::JoinHandle)."] pub trait JoinHandleExt < T > { # [doc = " Async version of [`JoinHandle::join()`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " - If called on the thread to join."] # [doc = " - If it was already polled to completion by another call to"] # [doc = "   [`JoinHandleExt::join_async()`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(all(target_feature = \"atomics\", not(unsupported_spawn)))]"] # [doc = " # wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);"] # [doc = " # #[cfg_attr(all(target_feature = \"atomics\", not(unsupported_spawn)), wasm_bindgen_test::wasm_bindgen_test)]"] # [doc = " # async fn test() {"] # [doc = " use web_thread::web::JoinHandleExt;"] # [doc = ""] # [doc = " web_thread::spawn(|| ()).join_async().await.unwrap();"] # [doc = " # }"] # [doc = " # #[cfg(not(all(target_feature = \"atomics\", not(unsupported_spawn))))]"] # [doc = " # let _ = test();"] # [doc = " ```"] fn join_async (& mut self) -> JoinHandleFuture < '_ , T > ; }
};
}
