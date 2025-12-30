// Generated macro for split_digest (function)
macro_rules! Depcrate_utilsplit_digest {
() => {
// Module: crate::util
// Provides: {"split_digest"}
// Dependencies: {}
# [doc = " Separates the digest into the FORS message, the Xmss tree index, and the Xmss leaf index."] pub (crate) fn split_digest < P : ForsParams > (digest : & Array < u8 , P :: M > ,) -> (& Array < u8 , P :: MD > , u64 , u32) { # [allow (deprecated)] let m = Array :: from_slice (& digest [.. P :: MD :: USIZE]) ; let idx_tree_size = (P :: H :: USIZE - P :: HPrime :: USIZE) . div_ceil (8) ; let idx_leaf_size = P :: HPrime :: USIZE . div_ceil (8) ; let mut idx_tree_bytes = [0u8 ; 8] ; let mut idx_leaf_bytes = [0u8 ; 4] ; idx_tree_bytes [8 - idx_tree_size ..] . copy_from_slice (& digest [P :: MD :: USIZE .. P :: MD :: USIZE + idx_tree_size]) ; idx_leaf_bytes [4 - idx_leaf_size ..] . copy_from_slice (& digest [P :: MD :: USIZE + idx_tree_size .. P :: MD :: USIZE + idx_tree_size + idx_leaf_size] ,) ; let mask : u64 = 1u64 . checked_shl (P :: H :: U32 - P :: HPrime :: U32) . unwrap_or (0) . wrapping_sub (1) ; let idx_tree = u64 :: from_be_bytes (idx_tree_bytes) & mask ; let idx_leaf = u32 :: from_be_bytes (idx_leaf_bytes) & ((1 << P :: HPrime :: USIZE) - 1) ; (m , idx_tree , idx_leaf) }
};
}
