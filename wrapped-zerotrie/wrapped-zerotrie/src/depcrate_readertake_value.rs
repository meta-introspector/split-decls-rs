// Generated macro for take_value (function)
macro_rules! Depcrate_readertake_value {
() => {
// Module: crate::reader
// Provides: {"take_value"}
// Dependencies: {}
# [doc = " Steps one node into the trie if the head node is a value node, returning the value."] # [doc = " If the head node is not a value node, no change is made."] # [doc = ""] # [doc = " The input-output argument `trie` starts at the original trie and ends pointing to"] # [doc = " the sub-trie with the value node removed."] pub (crate) fn take_value (trie : & mut & [u8]) -> Option < usize > { let (b , new_trie) = trie . split_first () ? ; match byte_type (* b) { NodeType :: Ascii | NodeType :: Span | NodeType :: Branch => None , NodeType :: Value => { let x ; (x , * trie) = read_varint_meta3 (* b , new_trie) ; Some (x) } } }
};
}
