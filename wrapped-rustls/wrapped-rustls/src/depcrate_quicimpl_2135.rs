// Generated macro for impl_2135 (impl)
macro_rules! Depcrate_quicimpl_2135 {
() => {
// Module: crate::quic
// Provides: {"impl_2135"}
// Dependencies: {}
impl Suite { # [doc = " Produce a set of initial keys given the connection ID, side and version"] pub fn keys (& self , client_dst_connection_id : & [u8] , side : Side , version : Version) -> Keys { Keys :: initial (version , self . suite , self . quic , client_dst_connection_id , side ,) } }
};
}
