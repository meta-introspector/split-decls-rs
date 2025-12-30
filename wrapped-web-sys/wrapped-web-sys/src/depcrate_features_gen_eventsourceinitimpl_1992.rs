// Generated macro for impl_1992 (impl)
macro_rules! Depcrate_features_gen_EventSourceInitimpl_1992 {
() => {
// Module: crate::features::gen_EventSourceInit
// Provides: {"impl_1992"}
// Dependencies: {}
impl EventSourceInit { # [doc = "Construct a new `EventSourceInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_with_credentials()` instead."] pub fn with_credentials (& mut self , val : bool) -> & mut Self { self . set_with_credentials (val) ; self } }
};
}
