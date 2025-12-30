// Generated macro for impl_720 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_720 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_720"}
// Dependencies: {}
impl < Side : SideData > fmt :: Debug for ConnectionState < '_ , '_ , Side > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: ReadTraffic (..) => f . debug_tuple ("ReadTraffic") . finish () , Self :: PeerClosed => write ! (f , "PeerClosed") , Self :: Closed => write ! (f , "Closed") , Self :: ReadEarlyData (..) => f . debug_tuple ("ReadEarlyData") . finish () , Self :: EncodeTlsData (..) => f . debug_tuple ("EncodeTlsData") . finish () , Self :: TransmitTlsData (..) => f . debug_tuple ("TransmitTlsData") . finish () , Self :: BlockedHandshake => f . debug_tuple ("BlockedHandshake") . finish () , Self :: WriteTraffic (..) => f . debug_tuple ("WriteTraffic") . finish () , } } }
};
}
