// Generated macro for impl_554 (impl)
macro_rules! Depcrate_quic_io_connection_stageimpl_554 {
() => {
// Module: crate::quic::io::connection_stage
// Provides: {"impl_554"}
// Dependencies: {}
impl Handshake { fn check_handshake_timeout_expired (& self , conn : & mut QuicheConnection ,) -> QuicResult < () > { if self . handshake_info . is_expired () { let _ = conn . close (false , quiche :: WireErrorCode :: ApplicationError as u64 , & [] ,) ; return Err (HandshakeError :: Timeout . into ()) ; } Ok (()) } }
};
}
