// Generated macro for impl_711 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_711 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_711"}
// Dependencies: {}
impl UnbufferedConnectionCommon < ClientConnectionData > { # [doc = " Processes the TLS records in `incoming_tls` buffer until a new [`UnbufferedStatus`] is"] # [doc = " reached."] pub fn process_tls_records < 'c , 'i > (& 'c mut self , incoming_tls : & 'i mut [u8] ,) -> UnbufferedStatus < 'c , 'i , ClientConnectionData > { self . process_tls_records_common (incoming_tls , | _ | false , | _ , _ | unreachable ! ()) } }
};
}
