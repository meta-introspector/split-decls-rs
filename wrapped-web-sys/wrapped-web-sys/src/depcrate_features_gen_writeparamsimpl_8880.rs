// Generated macro for impl_8880 (impl)
macro_rules! Depcrate_features_gen_WriteParamsimpl_8880 {
() => {
// Module: crate::features::gen_WriteParams
// Provides: {"impl_8880"}
// Dependencies: {}
impl WriteParams { # [cfg (feature = "WriteCommandType")] # [doc = "Construct a new `WriteParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"] pub fn new (type_ : WriteCommandType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_type (type_) ; ret } # [deprecated = "Use `set_data()` instead."] pub fn data (& mut self , val : Option < & :: wasm_bindgen :: JsValue >) -> & mut Self { self . set_data (val . unwrap_or (& :: wasm_bindgen :: JsValue :: NULL)) ; self } # [deprecated = "Use `set_position()` instead."] pub fn position (& mut self , val : Option < f64 >) -> & mut Self { self . set_position (val) ; self } # [deprecated = "Use `set_size()` instead."] pub fn size (& mut self , val : Option < f64 >) -> & mut Self { self . set_size (val) ; self } # [cfg (feature = "WriteCommandType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : WriteCommandType) -> & mut Self { self . set_type (val) ; self } }
};
}
