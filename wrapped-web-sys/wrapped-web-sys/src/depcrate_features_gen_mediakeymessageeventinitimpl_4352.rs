// Generated macro for impl_4352 (impl)
macro_rules! Depcrate_features_gen_MediaKeyMessageEventInitimpl_4352 {
() => {
// Module: crate::features::gen_MediaKeyMessageEventInit
// Provides: {"impl_4352"}
// Dependencies: {}
impl MediaKeyMessageEventInit { # [cfg (feature = "MediaKeyMessageType")] # [doc = "Construct a new `MediaKeyMessageEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"] pub fn new (message : & :: js_sys :: ArrayBuffer , message_type : MediaKeyMessageType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_message (message) ; ret . set_message_type (message_type) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_message()` instead."] pub fn message (& mut self , val : & :: js_sys :: ArrayBuffer) -> & mut Self { self . set_message (val) ; self } # [cfg (feature = "MediaKeyMessageType")] # [deprecated = "Use `set_message_type()` instead."] pub fn message_type (& mut self , val : MediaKeyMessageType) -> & mut Self { self . set_message_type (val) ; self } }
};
}
