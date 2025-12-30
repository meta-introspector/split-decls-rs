// Generated macro for impl_997 (impl)
macro_rules! Depcrate_features_gen_CloseEventInitimpl_997 {
() => {
// Module: crate::features::gen_CloseEventInit
// Provides: {"impl_997"}
// Dependencies: {}
impl CloseEventInit { # [doc = "Construct a new `CloseEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_code()` instead."] pub fn code (& mut self , val : u16) -> & mut Self { self . set_code (val) ; self } # [deprecated = "Use `set_reason()` instead."] pub fn reason (& mut self , val : & str) -> & mut Self { self . set_reason (val) ; self } # [deprecated = "Use `set_was_clean()` instead."] pub fn was_clean (& mut self , val : bool) -> & mut Self { self . set_was_clean (val) ; self } }
};
}
