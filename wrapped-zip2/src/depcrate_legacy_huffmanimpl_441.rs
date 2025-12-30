// Generated macro for impl_441 (impl)
macro_rules! Depcrate_legacy_huffmanimpl_441 {
() => {
// Module: crate::legacy::huffman
// Provides: {"impl_441"}
// Dependencies: {}
impl Default for HuffmanDecoder { fn default () -> Self { Self { table : [TableEntry :: default () ; 1 << HUFFMAN_LOOKUP_TABLE_BITS] , sentinel_bits : [0 ; MAX_HUFFMAN_BITS + 1] , offset_first_sym_idx : [0 ; MAX_HUFFMAN_BITS + 1] , syms : [0 ; MAX_HUFFMAN_SYMBOLS] , } } }
};
}
