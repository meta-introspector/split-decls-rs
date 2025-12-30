// Generated macro for impl_463 (impl)
macro_rules! Depcrate_quic_connection_mapimpl_463 {
() => {
// Module: crate::quic::connection::map
// Provides: {"impl_463"}
// Dependencies: {}
impl From < & ConnectionId < '_ > > for CidOwned { # [inline (always)] fn from (value : & ConnectionId < '_ >) -> Self { if value . len () > CONN_ID_USABLE_LEN { return CidOwned :: Generic (value . as_ref () . into ()) ; } let mut cid = [0 ; MAX_CONN_ID_QUADS] ; value . chunks (U64_SZ) . map (| c | match c . try_into () { Ok (v) => u64 :: from_le_bytes (v) , Err (_) => { let mut remainder = [0u8 ; U64_SZ] ; remainder [.. c . len ()] . copy_from_slice (c) ; u64 :: from_le_bytes (remainder) } , }) . enumerate () . for_each (| (i , v) | cid [i] = v) ; * cid . last_mut () . unwrap () |= (value . len () as u64) << 56 ; CidOwned :: Optimized (cid) } }
};
}
