// Generated macro for other_5 (other)
macro_rules! Depcrate_callbackother_5 {
() => {
// Module: crate::callback
// Provides: {"other_5"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (js_name = "setTimeout" , catch)] fn set_timeout (handler : & Function , timeout : i32) -> Result < JsValue , JsValue > ; # [wasm_bindgen (js_name = "setInterval" , catch)] fn set_interval (handler : & Function , timeout : i32) -> Result < JsValue , JsValue > ; # [wasm_bindgen (js_name = "clearTimeout")] fn clear_timeout (handle : JsValue) -> JsValue ; # [wasm_bindgen (js_name = "clearInterval")] fn clear_interval (handle : JsValue) -> JsValue ; }
};
}
