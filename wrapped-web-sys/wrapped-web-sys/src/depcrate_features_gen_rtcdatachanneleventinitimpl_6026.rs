// Generated macro for impl_6026 (impl)
macro_rules! Depcrate_features_gen_RtcDataChannelEventInitimpl_6026 {
() => {
// Module: crate::features::gen_RtcDataChannelEventInit
// Provides: {"impl_6026"}
// Dependencies: {}
impl RtcDataChannelEventInit { # [cfg (feature = "RtcDataChannel")] # [doc = "Construct a new `RtcDataChannelEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"] pub fn new (channel : & RtcDataChannel) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_channel (channel) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "RtcDataChannel")] # [deprecated = "Use `set_channel()` instead."] pub fn channel (& mut self , val : & RtcDataChannel) -> & mut Self { self . set_channel (val) ; self } }
};
}
