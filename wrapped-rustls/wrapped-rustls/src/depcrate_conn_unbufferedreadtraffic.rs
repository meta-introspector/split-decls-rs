// Generated macro for ReadTraffic (struct)
macro_rules! Depcrate_conn_unbufferedReadTraffic {
() => {
// Module: crate::conn::unbuffered
// Provides: {"ReadTraffic"}
// Dependencies: {}
# [doc = " Application data is available"] pub struct ReadTraffic < 'c , 'i , Side : SideData > { conn : & 'c mut UnbufferedConnectionCommon < Side > , _incoming_tls : & 'i mut [u8] , chunk : Option < Vec < u8 > > , }
};
}
