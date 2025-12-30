// Generated macro for impl_3918 (impl)
macro_rules! Depcrate_features_gen_IdbVersionChangeEventInitimpl_3918 {
() => {
// Module: crate::features::gen_IdbVersionChangeEventInit
// Provides: {"impl_3918"}
// Dependencies: {}
impl IdbVersionChangeEventInit { # [doc = "Construct a new `IdbVersionChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_new_version()` instead."] pub fn new_version (& mut self , val : Option < f64 >) -> & mut Self { self . set_new_version (val) ; self } # [deprecated = "Use `set_old_version()` instead."] pub fn old_version (& mut self , val : f64) -> & mut Self { self . set_old_version (val) ; self } }
};
}
