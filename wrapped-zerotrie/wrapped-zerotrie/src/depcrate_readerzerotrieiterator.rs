// Generated macro for ZeroTrieIterator (struct)
macro_rules! Depcrate_readerZeroTrieIterator {
() => {
// Module: crate::reader
// Provides: {"ZeroTrieIterator"}
// Dependencies: {}
# [doc = " Iterator type for walking the byte sequences contained in a ZeroTrie."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] # [derive (Debug)] pub struct ZeroTrieIterator < 'a > { # [doc = " Whether the PHF is enabled on this trie."] use_phf : bool , # [doc = " Intermediate state during iteration:"] # [doc = " 1. A trie (usually a slice of the original, bigger trie)"] # [doc = " 2. The string that leads to the trie"] # [doc = " 3. If the trie's lead node is a branch node, the current index being evaluated"] state : Vec < (& 'a [u8] , Vec < u8 > , usize) > , }
};
}
