// Generated macro for impl_2077 (impl)
macro_rules! Depcrate_features_gen_ExtendableEventInitimpl_2077 {
() => {
// Module: crate::features::gen_ExtendableEventInit
// Provides: {"impl_2077"}
// Dependencies: {}
impl ExtendableEventInit { # [doc = "Construct a new `ExtendableEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ExtendableEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } }
};
}
