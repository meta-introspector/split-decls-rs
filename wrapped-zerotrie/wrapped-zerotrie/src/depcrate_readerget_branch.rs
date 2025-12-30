// Generated macro for get_branch (function)
macro_rules! Depcrate_readerget_branch {
() => {
// Module: crate::reader
// Provides: {"get_branch"}
// Dependencies: {}
# [doc = " Given a slice starting with an offset table, returns the trie for the given index."] # [doc = ""] # [doc = " Arguments:"] # [doc = " - `trie` = a trie pointing at an offset table (after the branch node and search table)"] # [doc = " - `i` = the desired index within the offset table"] # [doc = " - `n` = the number of items in the offset table"] # [doc = " - `w` = the width of the offset table items minus one"] # [inline] fn get_branch (mut trie : & [u8] , i : usize , n : usize , mut w : usize) -> & [u8] { let mut p = 0usize ; let mut q = 0usize ; loop { let indices ; (indices , trie) = trie . debug_split_at (n - 1) ; p = (p << 8) + if i == 0 { 0 } else { * indices . get (i - 1) . debug_unwrap_or (& 0) as usize } ; q = match indices . get (i) { Some (x) => (q << 8) + * x as usize , None => trie . len () , } ; if w == 0 { break ; } w -= 1 ; } trie . get (p .. q) . debug_unwrap_or (& []) }
};
}
