// Generated macro for impl_148 (impl)
macro_rules! Depcrate_features_gen_AnimationPropertyValueDetailsimpl_148 {
() => {
// Module: crate::features::gen_AnimationPropertyValueDetails
// Provides: {"impl_148"}
// Dependencies: {}
impl AnimationPropertyValueDetails { # [cfg (feature = "CompositeOperation")] # [doc = "Construct a new `AnimationPropertyValueDetails`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"] pub fn new (composite : CompositeOperation , offset : f64) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_composite (composite) ; ret . set_offset (offset) ; ret } # [cfg (feature = "CompositeOperation")] # [deprecated = "Use `set_composite()` instead."] pub fn composite (& mut self , val : CompositeOperation) -> & mut Self { self . set_composite (val) ; self } # [deprecated = "Use `set_easing()` instead."] pub fn easing (& mut self , val : & str) -> & mut Self { self . set_easing (val) ; self } # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : f64) -> & mut Self { self . set_offset (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & str) -> & mut Self { self . set_value (val) ; self } }
};
}
