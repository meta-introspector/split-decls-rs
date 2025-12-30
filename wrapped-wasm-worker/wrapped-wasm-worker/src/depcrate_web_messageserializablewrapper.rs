// Generated macro for SerializableWrapper (struct)
macro_rules! Depcrate_web_messageSerializableWrapper {
() => {
// Module: crate::web::message
// Provides: {"SerializableWrapper"}
// Dependencies: {}
# [doc = " Wrapper that implements [`MessageSend`] for values implementing"] # [doc = " [`Serializable`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(all(target_feature = \"atomics\", not(unsupported_spawn)))]"] # [doc = " # wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);"] # [doc = " # #[cfg_attr(all(target_feature = \"atomics\", not(unsupported_spawn)), wasm_bindgen_test::wasm_bindgen_test)]"] # [doc = " # async fn test() {"] # [doc = " use js_sys::ArrayBuffer;"] # [doc = " use web_thread::web::{self, JoinHandleExt};"] # [doc = " use web_thread::web::message::SerializableWrapper;"] # [doc = ""] # [doc = " let message = SerializableWrapper(ArrayBuffer::new(1000));"] # [doc = " web::spawn_with_message("] # [doc = " \t|message| async move {"] # [doc = " \t\t// Do work."] # [doc = " #   \tlet _ = message;"] # [doc = " \t},"] # [doc = " \tmessage,"] # [doc = " )"] # [doc = " .join_async()"] # [doc = " .await"] # [doc = " .unwrap();"] # [doc = " # }"] # [doc = " # #[cfg(not(all(target_feature = \"atomics\", not(unsupported_spawn))))]"] # [doc = " # let _ = test();"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct SerializableWrapper < T > (pub T) where T : Into < JsValue > + JsCast + Serializable ;
};
}
