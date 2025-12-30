// Generated macro for compress_postfix_mid (function)
macro_rules! Depcrate_ownedcompress_postfix_mid {
() => {
// Module: crate::owned
// Provides: {"compress_postfix_mid"}
// Dependencies: {}
fn compress_postfix_mid (chunks : & [u8] , chunk_size : usize ,) -> Result < (Vec < u8 > , Vec < u8 >) > { let mut root = vec ! [] ; let mut children = vec ! [] ; let mut bychild = HashMap :: new () ; for i in 0 .. (chunks . len () / chunk_size) { let chunk = & chunks [i * chunk_size .. (i + 1) * chunk_size] ; if ! bychild . contains_key (chunk) { let start = bychild . len () ; if start > :: std :: u8 :: MAX as usize { return Err (Error :: GaveUp) ; } bychild . insert (chunk , start as u8) ; children . extend (chunk) ; } root . push (bychild [chunk]) ; } Ok ((root , children)) }
};
}
