// Generated macro for impl_636 (impl)
macro_rules! Depcrate_features_gen_BlockParsingOptionsimpl_636 {
() => {
// Module: crate::features::gen_BlockParsingOptions
// Provides: {"impl_636"}
// Dependencies: {}
impl BlockParsingOptions { # [doc = "Construct a new `BlockParsingOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_block_script_created()` instead."] pub fn block_script_created (& mut self , val : bool) -> & mut Self { self . set_block_script_created (val) ; self } }
};
}
