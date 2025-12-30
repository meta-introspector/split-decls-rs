// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_features_gen_CookieInitimpl_1220 {
() => {
// Module: crate::features::gen_CookieInit
// Provides: {"impl_1220"}
// Dependencies: {}
impl CookieInit { # [doc = "Construct a new `CookieInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CookieInit`*"] pub fn new (name : & str , value : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_value (value) ; ret } # [deprecated = "Use `set_domain()` instead."] pub fn domain (& mut self , val : Option < & str >) -> & mut Self { self . set_domain (val) ; self } # [deprecated = "Use `set_expires()` instead."] pub fn expires (& mut self , val : Option < f64 >) -> & mut Self { self . set_expires (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_partitioned()` instead."] pub fn partitioned (& mut self , val : bool) -> & mut Self { self . set_partitioned (val) ; self } # [deprecated = "Use `set_path()` instead."] pub fn path (& mut self , val : & str) -> & mut Self { self . set_path (val) ; self } # [cfg (feature = "CookieSameSite")] # [deprecated = "Use `set_same_site()` instead."] pub fn same_site (& mut self , val : CookieSameSite) -> & mut Self { self . set_same_site (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & str) -> & mut Self { self . set_value (val) ; self } }
};
}
