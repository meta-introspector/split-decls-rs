// Generated macro for impl_5186 (impl)
macro_rules! Depcrate_features_gen_Pbkdf2Paramsimpl_5186 {
() => {
// Module: crate::features::gen_Pbkdf2Params
// Provides: {"impl_5186"}
// Dependencies: {}
impl Pbkdf2Params { # [doc = "Construct a new `Pbkdf2Params`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"] pub fn new (name : & str , hash : & :: wasm_bindgen :: JsValue , iterations : u32 , salt : & :: js_sys :: Object ,) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret . set_iterations (iterations) ; ret . set_salt (salt) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } # [deprecated = "Use `set_iterations()` instead."] pub fn iterations (& mut self , val : u32) -> & mut Self { self . set_iterations (val) ; self } # [deprecated = "Use `set_salt()` instead."] pub fn salt (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_salt (val) ; self } }
};
}
