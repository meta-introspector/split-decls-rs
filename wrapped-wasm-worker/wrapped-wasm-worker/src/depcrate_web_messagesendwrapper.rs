// Generated macro for SendWrapper (struct)
macro_rules! Depcrate_web_messageSendWrapper {
() => {
// Module: crate::web::message
// Provides: {"SendWrapper"}
// Dependencies: {}
# [doc = " Wrapper that implements [`MessageSend`] for values implementing [`Send`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(all(target_feature = \"atomics\", not(unsupported_spawn)))]"] # [doc = " # wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);"] # [doc = " # #[cfg_attr(all(target_feature = \"atomics\", not(unsupported_spawn)), wasm_bindgen_test::wasm_bindgen_test)]"] # [doc = " # async fn test() {"] # [doc = " use std::sync::Arc;"] # [doc = " use web_thread::web::{self, JoinHandleExt};"] # [doc = " use web_thread::web::message::SendWrapper;"] # [doc = ""] # [doc = " let data = Arc::new(vec![0, 1, 2, 3, 4]);"] # [doc = " let message = SendWrapper(Arc::clone(&data));"] # [doc = " let mut handle = web::spawn_with_message("] # [doc = " \t|message| async move {"] # [doc = " \t\t// Do work."] # [doc = " #       drop(message);"] # [doc = " \t},"] # [doc = " \tmessage,"] # [doc = " );"] # [doc = ""] # [doc = " // Do work."] # [doc = " # /*"] # [doc = " data;"] # [doc = " # */"] # [doc = " # let _ =data;"] # [doc = ""] # [doc = " handle.join_async().await.unwrap();"] # [doc = " # }"] # [doc = " # #[cfg(not(all(target_feature = \"atomics\", not(unsupported_spawn))))]"] # [doc = " # let _ = test();"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct SendWrapper < T > (pub T) where T : Send ;
};
}
