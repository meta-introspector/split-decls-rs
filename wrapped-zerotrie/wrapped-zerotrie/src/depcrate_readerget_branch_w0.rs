// Generated macro for get_branch_w0 (function)
macro_rules! Depcrate_readerget_branch_w0 {
() => {
// Module: crate::reader
// Provides: {"get_branch_w0"}
// Dependencies: {}
# [doc = " Version of [`get_branch()`] specialized for the case `w == 0` for performance"] # [inline] fn get_branch_w0 (mut trie : & [u8] , i : usize , n : usize) -> & [u8] { let indices ; (indices , trie) = trie . debug_split_at (n - 1) ; let p = if i == 0 { 0 } else { * indices . get (i - 1) . debug_unwrap_or (& 0) as usize } ; let q = match indices . get (i) { Some (x) => * x as usize , None => trie . len () , } ; trie . get (p .. q) . debug_unwrap_or (& []) }
};
}
