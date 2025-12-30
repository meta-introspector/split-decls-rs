// Generated macro for impl_6226 (impl)
macro_rules! Depcrate_features_gen_RtcOfferOptionsimpl_6226 {
() => {
// Module: crate::features::gen_RtcOfferOptions
// Provides: {"impl_6226"}
// Dependencies: {}
impl RtcOfferOptions { # [doc = "Construct a new `RtcOfferOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_ice_restart()` instead."] pub fn ice_restart (& mut self , val : bool) -> & mut Self { self . set_ice_restart (val) ; self } # [deprecated = "Use `set_offer_to_receive_audio()` instead."] pub fn offer_to_receive_audio (& mut self , val : bool) -> & mut Self { self . set_offer_to_receive_audio (val) ; self } # [deprecated = "Use `set_offer_to_receive_video()` instead."] pub fn offer_to_receive_video (& mut self , val : bool) -> & mut Self { self . set_offer_to_receive_video (val) ; self } }
};
}
