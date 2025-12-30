// Generated macro for impl_365 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_365 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_365"}
// Dependencies: {}
impl NewSessionTicketPayload { pub (crate) fn new (lifetime_hint : Duration , ticket : Vec < u8 >) -> Self { Self { lifetime_hint , ticket : Arc :: new (PayloadU16 :: new (ticket)) , } } }
};
}
