// Generated macro for impl_6255 (impl)
macro_rules! Depcrate_features_gen_RtcPeerConnectionIceEventInitimpl_6255 {
() => {
// Module: crate::features::gen_RtcPeerConnectionIceEventInit
// Provides: {"impl_6255"}
// Dependencies: {}
impl RtcPeerConnectionIceEventInit { # [doc = "Construct a new `RtcPeerConnectionIceEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "RtcIceCandidate")] # [deprecated = "Use `set_candidate()` instead."] pub fn candidate (& mut self , val : Option < & RtcIceCandidate >) -> & mut Self { self . set_candidate (val) ; self } }
};
}
