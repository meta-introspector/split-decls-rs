// Generated macro for impl_304 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_304 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_304"}
// Dependencies: {}
impl HelloRetryRequest { fn payload_encode (& self , bytes : & mut Vec < u8 > , purpose : Encoding) { self . legacy_version . encode (bytes) ; HELLO_RETRY_REQUEST_RANDOM . encode (bytes) ; self . session_id . encode (bytes) ; self . cipher_suite . encode (bytes) ; Compression :: Null . encode (bytes) ; match purpose { Encoding :: EchConfirmation if self . extensions . encrypted_client_hello . is_some () => { let hrr_confirmation = [0u8 ; 8] ; HelloRetryRequestExtensions { encrypted_client_hello : Some (Payload :: Borrowed (& hrr_confirmation)) , .. self . extensions . clone () } . encode (bytes) ; } _ => self . extensions . encode (bytes) , } } }
};
}
