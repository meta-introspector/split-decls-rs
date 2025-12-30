// Generated macro for impl_1189 (impl)
macro_rules! Depcrate_features_gen_ConvertCoordinateOptionsimpl_1189 {
() => {
// Module: crate::features::gen_ConvertCoordinateOptions
// Provides: {"impl_1189"}
// Dependencies: {}
impl ConvertCoordinateOptions { # [doc = "Construct a new `ConvertCoordinateOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "CssBoxType")] # [deprecated = "Use `set_from_box()` instead."] pub fn from_box (& mut self , val : CssBoxType) -> & mut Self { self . set_from_box (val) ; self } # [cfg (feature = "CssBoxType")] # [deprecated = "Use `set_to_box()` instead."] pub fn to_box (& mut self , val : CssBoxType) -> & mut Self { self . set_to_box (val) ; self } }
};
}
