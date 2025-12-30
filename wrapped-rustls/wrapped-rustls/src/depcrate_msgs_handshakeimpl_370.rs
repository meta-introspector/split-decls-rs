// Generated macro for impl_370 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_370 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_370"}
// Dependencies: {}
impl NewSessionTicketPayloadTls13 { pub (crate) fn new (lifetime : Duration , age_add : u32 , nonce : [u8 ; 32] , ticket : Vec < u8 >) -> Self { Self { lifetime , age_add , nonce : PayloadU8 :: new (nonce . to_vec ()) , ticket : Arc :: new (PayloadU16 :: new (ticket)) , extensions : NewSessionTicketExtensions :: default () , } } }
};
}
