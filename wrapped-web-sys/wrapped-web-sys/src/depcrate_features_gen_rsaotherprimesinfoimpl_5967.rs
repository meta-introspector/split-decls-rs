// Generated macro for impl_5967 (impl)
macro_rules! Depcrate_features_gen_RsaOtherPrimesInfoimpl_5967 {
() => {
// Module: crate::features::gen_RsaOtherPrimesInfo
// Provides: {"impl_5967"}
// Dependencies: {}
impl RsaOtherPrimesInfo { # [doc = "Construct a new `RsaOtherPrimesInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"] pub fn new (d : & str , r : & str , t : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_d (d) ; ret . set_r (r) ; ret . set_t (t) ; ret } # [deprecated = "Use `set_d()` instead."] pub fn d (& mut self , val : & str) -> & mut Self { self . set_d (val) ; self } # [deprecated = "Use `set_r()` instead."] pub fn r (& mut self , val : & str) -> & mut Self { self . set_r (val) ; self } # [deprecated = "Use `set_t()` instead."] pub fn t (& mut self , val : & str) -> & mut Self { self . set_t (val) ; self } }
};
}
