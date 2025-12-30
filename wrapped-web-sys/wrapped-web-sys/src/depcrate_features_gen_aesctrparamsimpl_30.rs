// Generated macro for impl_30 (impl)
macro_rules! Depcrate_features_gen_AesCtrParamsimpl_30 {
() => {
// Module: crate::features::gen_AesCtrParams
// Provides: {"impl_30"}
// Dependencies: {}
impl AesCtrParams { # [doc = "Construct a new `AesCtrParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"] pub fn new (name : & str , counter : & :: js_sys :: Object , length : u8) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_counter (counter) ; ret . set_length (length) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_counter()` instead."] pub fn counter (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_counter (val) ; self } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u8) -> & mut Self { self . set_length (val) ; self } }
};
}
