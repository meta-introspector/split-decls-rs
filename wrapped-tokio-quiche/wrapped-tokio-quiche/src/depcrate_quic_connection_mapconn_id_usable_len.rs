// Generated macro for CONN_ID_USABLE_LEN (const)
macro_rules! Depcrate_quic_connection_mapCONN_ID_USABLE_LEN {
() => {
// Module: crate::quic::connection::map
// Provides: {"CONN_ID_USABLE_LEN"}
// Dependencies: {}
const CONN_ID_USABLE_LEN : usize = min_usize (MAX_CONN_ID_QUADS * U64_SZ - 1 , min_usize (MAX_CONN_ID_LEN , u8 :: MAX as _) ,) ;
};
}
