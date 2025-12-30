// Generated macro for impl_6032 (impl)
macro_rules! Depcrate_features_gen_RtcDataChannelInitimpl_6032 {
() => {
// Module: crate::features::gen_RtcDataChannelInit
// Provides: {"impl_6032"}
// Dependencies: {}
impl RtcDataChannelInit { # [doc = "Construct a new `RtcDataChannelInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : u16) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_max_packet_life_time()` instead."] pub fn max_packet_life_time (& mut self , val : u16) -> & mut Self { self . set_max_packet_life_time (val) ; self } # [deprecated = "Use `set_max_retransmit_time()` instead."] pub fn max_retransmit_time (& mut self , val : u16) -> & mut Self { self . set_max_retransmit_time (val) ; self } # [deprecated = "Use `set_max_retransmits()` instead."] pub fn max_retransmits (& mut self , val : u16) -> & mut Self { self . set_max_retransmits (val) ; self } # [deprecated = "Use `set_negotiated()` instead."] pub fn negotiated (& mut self , val : bool) -> & mut Self { self . set_negotiated (val) ; self } # [deprecated = "Use `set_ordered()` instead."] pub fn ordered (& mut self , val : bool) -> & mut Self { self . set_ordered (val) ; self } # [deprecated = "Use `set_protocol()` instead."] pub fn protocol (& mut self , val : & str) -> & mut Self { self . set_protocol (val) ; self } }
};
}
