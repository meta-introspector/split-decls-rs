// Generated macro for short_dcid (function)
macro_rules! Depcrate_quic_routershort_dcid {
() => {
// Module: crate::quic::router
// Provides: {"short_dcid"}
// Dependencies: {}
fn short_dcid (buf : & [u8]) -> Option < ConnectionId < '_ > > { let is_short_dcid = buf . first () ? >> 7 == 0 ; if is_short_dcid { buf . get (1 .. 1 + MAX_CONN_ID_LEN) . map (ConnectionId :: from_ref) } else { None } }
};
}
