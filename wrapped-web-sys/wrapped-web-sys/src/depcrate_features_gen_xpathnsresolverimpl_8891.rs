// Generated macro for impl_8891 (impl)
macro_rules! Depcrate_features_gen_XPathNsResolverimpl_8891 {
() => {
// Module: crate::features::gen_XPathNsResolver
// Provides: {"impl_8891"}
// Dependencies: {}
impl XPathNsResolver { # [doc = "Construct a new `XPathNsResolver`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_lookup_namespace_uri()` instead."] pub fn lookup_namespace_uri (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_lookup_namespace_uri (val) ; self } }
};
}
