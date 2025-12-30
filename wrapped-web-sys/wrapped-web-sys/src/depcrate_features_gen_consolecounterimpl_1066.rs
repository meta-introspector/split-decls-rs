// Generated macro for impl_1066 (impl)
macro_rules! Depcrate_features_gen_ConsoleCounterimpl_1066 {
() => {
// Module: crate::features::gen_ConsoleCounter
// Provides: {"impl_1066"}
// Dependencies: {}
impl ConsoleCounter { # [doc = "Construct a new `ConsoleCounter`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_count()` instead."] pub fn count (& mut self , val : u32) -> & mut Self { self . set_count (val) ; self } # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } }
};
}
