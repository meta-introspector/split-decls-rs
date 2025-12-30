// Generated macro for InboundHeaders (struct)
macro_rules! Depcrate_http3_driver_hooksInboundHeaders {
() => {
// Module: crate::http3::driver::hooks
// Provides: {"InboundHeaders"}
// Dependencies: {}
# [doc = " A HEADERS frame received from the [`h3::Connection`], to be processed by"] # [doc = " the [DriverHooks]."] pub (crate) struct InboundHeaders { pub (crate) stream_id : u64 , pub (crate) headers : Vec < h3 :: Header > , pub (crate) has_body : bool , }
};
}
