macro_rules! deps {
    () => {
        TableEntry!();
        HuffmanDecoder!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl Default for HuffmanDecoder { fn default () -> Self { Self { table : [TableEntry :: default () ; 1 << HUFFMAN_LOOKUP_TABLE_BITS] , sentinel_bits : [0 ; MAX_HUFFMAN_BITS + 1] , offset_first_sym_idx : [0 ; MAX_HUFFMAN_BITS + 1] , syms : [0 ; MAX_HUFFMAN_SYMBOLS] , } } }
    };
}

impl_291!();