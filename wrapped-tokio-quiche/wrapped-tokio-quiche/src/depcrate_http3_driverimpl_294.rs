// Generated macro for impl_294 (impl)
macro_rules! Depcrate_http3_driverimpl_294 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_294"}
// Dependencies: {}
impl fmt :: Debug for IncomingH3Headers { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IncomingH3Headers") . field ("stream_id" , & self . stream_id) . field ("headers" , & self . headers) . field ("read_fin" , & self . read_fin) . field ("h3_audit_stats" , & self . h3_audit_stats) . finish () } }
};
}
