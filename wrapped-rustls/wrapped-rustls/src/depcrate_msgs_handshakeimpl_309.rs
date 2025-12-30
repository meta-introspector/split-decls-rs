// Generated macro for impl_309 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_309 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_309"}
// Dependencies: {}
impl ServerHelloPayload { fn payload_encode (& self , bytes : & mut Vec < u8 > , encoding : Encoding) { debug_assert ! (! matches ! (encoding , Encoding :: EchConfirmation) , "we cannot compute an ECH confirmation on a received ServerHello") ; self . legacy_version . encode (bytes) ; self . random . encode (bytes) ; self . session_id . encode (bytes) ; self . cipher_suite . encode (bytes) ; self . compression_method . encode (bytes) ; self . extensions . encode (bytes) ; } }
};
}
