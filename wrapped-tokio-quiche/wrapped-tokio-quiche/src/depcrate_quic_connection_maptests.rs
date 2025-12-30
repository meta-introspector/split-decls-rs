// Generated macro for tests (module)
macro_rules! Depcrate_quic_connection_maptests {
() => {
// Module: crate::quic::connection::map
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use quiche :: ConnectionId ; # [test] fn cid_storage () { let max_v1_cid = ConnectionId :: from_ref (& [0xfa ; MAX_CONN_ID_LEN]) ; let optimized = CidOwned :: from (& max_v1_cid) ; assert ! (matches ! (optimized , CidOwned :: Optimized (_)) , "QUIC v1 CID is not stored inline") ; let oversize_cid = ConnectionId :: from_ref (& [0x1b ; MAX_CONN_ID_LEN + 20]) ; let boxed = CidOwned :: from (& oversize_cid) ; assert ! (matches ! (boxed , CidOwned :: Generic (_)) , "Oversized CID is not boxed") ; } }
};
}
