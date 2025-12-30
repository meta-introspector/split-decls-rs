// Generated macro for impl_712 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_712 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_712"}
// Dependencies: {}
impl UnbufferedConnectionCommon < ServerConnectionData > { # [doc = " Processes the TLS records in `incoming_tls` buffer until a new [`UnbufferedStatus`] is"] # [doc = " reached."] pub fn process_tls_records < 'c , 'i > (& 'c mut self , incoming_tls : & 'i mut [u8] ,) -> UnbufferedStatus < 'c , 'i , ServerConnectionData > { self . process_tls_records_common (incoming_tls , | conn | conn . peek_early_data () . is_some () , | conn , incoming_tls | ReadEarlyData :: new (conn , incoming_tls) . into () ,) } }
};
}
