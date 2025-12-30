// Generated macro for compress_postfix_leaves (function)
macro_rules! Depcrate_ownedcompress_postfix_leaves {
() => {
// Module: crate::owned
// Provides: {"compress_postfix_leaves"}
// Dependencies: {}
fn compress_postfix_leaves (chunks : & [u64]) -> Result < (Vec < u8 > , Vec < u64 >) > { let mut root = vec ! [] ; let mut children = vec ! [] ; let mut bychild = HashMap :: new () ; for & chunk in chunks { if ! bychild . contains_key (& chunk) { let start = bychild . len () ; if start > :: std :: u8 :: MAX as usize { return Err (Error :: GaveUp) ; } bychild . insert (chunk , start as u8) ; children . push (chunk) ; } root . push (bychild [& chunk]) ; } Ok ((root , children)) }
};
}
