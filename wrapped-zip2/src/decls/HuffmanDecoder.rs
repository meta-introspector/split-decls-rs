macro_rules! deps {
    () => {
        TableEntry!();
    };
}

macro_rules! HuffmanDecoder {
    () => {
        deps!();
        pub struct HuffmanDecoder { # [doc = " Lookup table for fast decoding of short codewords."] pub table : [TableEntry ; 1 << HUFFMAN_LOOKUP_TABLE_BITS] , # [doc = " \"Sentinel bits\" value for each codeword length."] pub sentinel_bits : [u32 ; MAX_HUFFMAN_BITS + 1] , # [doc = " First symbol index minus first codeword mod 2**16 for each length."] pub offset_first_sym_idx : [u16 ; MAX_HUFFMAN_BITS + 1] , # [doc = " Map from symbol index to symbol."] pub syms : [u16 ; MAX_HUFFMAN_SYMBOLS] , }
    };
}

HuffmanDecoder!()