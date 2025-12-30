// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_features_gen_CookieStoreDeleteOptionsimpl_1242 {
() => {
// Module: crate::features::gen_CookieStoreDeleteOptions
// Provides: {"impl_1242"}
// Dependencies: {}
impl CookieStoreDeleteOptions { # [doc = "Construct a new `CookieStoreDeleteOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_domain()` instead."] pub fn domain (& mut self , val : Option < & str >) -> & mut Self { self . set_domain (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_partitioned()` instead."] pub fn partitioned (& mut self , val : bool) -> & mut Self { self . set_partitioned (val) ; self } # [deprecated = "Use `set_path()` instead."] pub fn path (& mut self , val : & str) -> & mut Self { self . set_path (val) ; self } }
};
}
