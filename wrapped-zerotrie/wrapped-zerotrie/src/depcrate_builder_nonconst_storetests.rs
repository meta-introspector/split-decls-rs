// Generated macro for tests (module)
macro_rules! Depcrate_builder_nonconst_storetests {
() => {
// Module: crate::builder::nonconst::store
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_swap_ranges () { let s = b"..abcdefghijkl=" ; let mut s = s . iter () . copied () . collect :: < VecDeque < u8 > > () ; s . atbs_swap_ranges (2 , 7 , 14) ; assert_eq ! (s . atbs_to_bytes () , b"..fghijklabcde=") ; } }
};
}
