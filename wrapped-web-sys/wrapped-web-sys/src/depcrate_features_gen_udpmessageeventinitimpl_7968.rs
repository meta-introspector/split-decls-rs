// Generated macro for impl_7968 (impl)
macro_rules! Depcrate_features_gen_UdpMessageEventInitimpl_7968 {
() => {
// Module: crate::features::gen_UdpMessageEventInit
// Provides: {"impl_7968"}
// Dependencies: {}
impl UdpMessageEventInit { # [doc = "Construct a new `UdpMessageEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_data()` instead."] pub fn data (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_data (val) ; self } # [deprecated = "Use `set_remote_address()` instead."] pub fn remote_address (& mut self , val : & str) -> & mut Self { self . set_remote_address (val) ; self } # [deprecated = "Use `set_remote_port()` instead."] pub fn remote_port (& mut self , val : u16) -> & mut Self { self . set_remote_port (val) ; self } }
};
}
