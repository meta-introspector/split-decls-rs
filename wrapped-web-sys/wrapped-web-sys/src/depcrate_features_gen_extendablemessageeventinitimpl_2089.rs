// Generated macro for impl_2089 (impl)
macro_rules! Depcrate_features_gen_ExtendableMessageEventInitimpl_2089 {
() => {
// Module: crate::features::gen_ExtendableMessageEventInit
// Provides: {"impl_2089"}
// Dependencies: {}
impl ExtendableMessageEventInit { # [doc = "Construct a new `ExtendableMessageEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_data()` instead."] pub fn data (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_data (val) ; self } # [deprecated = "Use `set_last_event_id()` instead."] pub fn last_event_id (& mut self , val : & str) -> & mut Self { self . set_last_event_id (val) ; self } # [deprecated = "Use `set_origin()` instead."] pub fn origin (& mut self , val : & str) -> & mut Self { self . set_origin (val) ; self } # [deprecated = "Use `set_ports()` instead."] pub fn ports (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_ports (val) ; self } # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_source (val) ; self } }
};
}
