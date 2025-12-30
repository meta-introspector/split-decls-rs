// Generated macro for NewSessionTicketPayloadTls13 (struct)
macro_rules! Depcrate_msgs_handshakeNewSessionTicketPayloadTls13 {
() => {
// Module: crate::msgs::handshake
// Provides: {"NewSessionTicketPayloadTls13"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct NewSessionTicketPayloadTls13 { pub (crate) lifetime : Duration , pub (crate) age_add : u32 , pub (crate) nonce : PayloadU8 , pub (crate) ticket : Arc < PayloadU16 > , pub (crate) extensions : NewSessionTicketExtensions , }
};
}
