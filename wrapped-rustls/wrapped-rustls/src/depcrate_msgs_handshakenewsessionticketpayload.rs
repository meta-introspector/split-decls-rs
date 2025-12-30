// Generated macro for NewSessionTicketPayload (struct)
macro_rules! Depcrate_msgs_handshakeNewSessionTicketPayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"NewSessionTicketPayload"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct NewSessionTicketPayload { pub (crate) lifetime_hint : Duration , pub (crate) ticket : Arc < PayloadU16 > , }
};
}
