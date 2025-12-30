// Generated macro for impl_1623 (impl)
macro_rules! Depcrate_features_gen_DnsCacheEntryimpl_1623 {
() => {
// Module: crate::features::gen_DnsCacheEntry
// Provides: {"impl_1623"}
// Dependencies: {}
impl DnsCacheEntry { # [doc = "Construct a new `DnsCacheEntry`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_expiration()` instead."] pub fn expiration (& mut self , val : f64) -> & mut Self { self . set_expiration (val) ; self } # [deprecated = "Use `set_family()` instead."] pub fn family (& mut self , val : & str) -> & mut Self { self . set_family (val) ; self } # [deprecated = "Use `set_hostaddr()` instead."] pub fn hostaddr (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hostaddr (val) ; self } # [deprecated = "Use `set_hostname()` instead."] pub fn hostname (& mut self , val : & str) -> & mut Self { self . set_hostname (val) ; self } # [deprecated = "Use `set_trr()` instead."] pub fn trr (& mut self , val : bool) -> & mut Self { self . set_trr (val) ; self } }
};
}
