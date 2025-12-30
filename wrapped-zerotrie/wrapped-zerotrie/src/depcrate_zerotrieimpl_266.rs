// Generated macro for impl_266 (impl)
macro_rules! Depcrate_zerotrieimpl_266 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_266"}
// Dependencies: {}
impl < Store > ZeroTrie < Store > where Store : AsRef < [u8] > , { # [doc = " Queries the trie for a string."] pub fn get < K > (& self , key : K) -> Option < usize > where K : AsRef < [u8] > , { impl_dispatch ! (& self , get (key)) } # [doc = " Returns `true` if the trie is empty."] pub fn is_empty (& self) -> bool { impl_dispatch ! (& self , is_empty ()) } # [doc = " Returns the size of the trie in number of bytes."] # [doc = ""] # [doc = " To get the number of keys in the trie, use `.iter().count()`."] pub fn byte_len (& self) -> usize { impl_dispatch ! (& self , byte_len ()) } }
};
}
