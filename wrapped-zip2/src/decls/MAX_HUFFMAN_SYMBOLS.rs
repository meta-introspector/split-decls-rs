macro_rules! MAX_HUFFMAN_SYMBOLS {
    () => {
        # [doc = " Maximum number of symbols in a Huffman table (256 for literals, 64 for lengths/distances)"] const MAX_HUFFMAN_SYMBOLS : usize = 1 << 8 ;
    };
}

MAX_HUFFMAN_SYMBOLS!();