// Generated macro for CidOwned (enum)
macro_rules! Depcrate_quic_connection_mapCidOwned {
() => {
// Module: crate::quic::connection::map
// Provides: {"CidOwned"}
// Dependencies: {}
# [doc = " A non unique connection identifier, multiple Cids can map to the same"] # [doc = " conenction."] # [derive (PartialEq , Eq , PartialOrd , Ord)] enum CidOwned { # [doc = " The QUIC connections IDs theoretically have unbounded length, so for the"] # [doc = " generic case a boxed slice is used to store the ID."] Generic (Box < [u8] >) , # [doc = " For QUIC version 1 (the one that actually exists) the maximal ID size is"] # [doc = " `20`, which should correspond to the `MAX_CONN_ID_LEN` value. For"] # [doc = " this common case, we store the ID in a u64 array for faster"] # [doc = " comparison (and therefore BTreeMap lookups)."] Optimized ([u64 ; MAX_CONN_ID_QUADS]) , }
};
}
