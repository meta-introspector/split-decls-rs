// Generated macro for impl_7916 (impl)
macro_rules! Depcrate_features_gen_TreeCellInfoimpl_7916 {
() => {
// Module: crate::features::gen_TreeCellInfo
// Provides: {"impl_7916"}
// Dependencies: {}
impl TreeCellInfo { # [doc = "Construct a new `TreeCellInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_child_elt()` instead."] pub fn child_elt (& mut self , val : & str) -> & mut Self { self . set_child_elt (val) ; self } # [deprecated = "Use `set_row()` instead."] pub fn row (& mut self , val : i32) -> & mut Self { self . set_row (val) ; self } }
};
}
