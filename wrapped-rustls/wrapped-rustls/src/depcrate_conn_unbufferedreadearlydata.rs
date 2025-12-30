// Generated macro for ReadEarlyData (struct)
macro_rules! Depcrate_conn_unbufferedReadEarlyData {
() => {
// Module: crate::conn::unbuffered
// Provides: {"ReadEarlyData"}
// Dependencies: {}
# [doc = " Early application-data is available."] pub struct ReadEarlyData < 'c , 'i , Side : SideData > { conn : & 'c mut UnbufferedConnectionCommon < Side > , _incoming_tls : & 'i mut [u8] , chunk : Option < Vec < u8 > > , }
};
}
