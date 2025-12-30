// Generated macro for impl_4905 (impl)
macro_rules! Depcrate_features_gen_NodeFilterimpl_4905 {
() => {
// Module: crate::features::gen_NodeFilter
// Provides: {"impl_4905"}
// Dependencies: {}
impl NodeFilter { # [doc = "Construct a new `NodeFilter`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `NodeFilter`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_accept_node()` instead."] pub fn accept_node (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_accept_node (val) ; self } }
};
}
