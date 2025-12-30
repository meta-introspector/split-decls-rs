// Generated macro for EncodeTlsData (struct)
macro_rules! Depcrate_conn_unbufferedEncodeTlsData {
() => {
// Module: crate::conn::unbuffered
// Provides: {"EncodeTlsData"}
// Dependencies: {}
# [doc = " A handshake record must be encoded"] pub struct EncodeTlsData < 'c , Side : SideData > { conn : & 'c mut UnbufferedConnectionCommon < Side > , chunk : Option < Vec < u8 > > , }
};
}
