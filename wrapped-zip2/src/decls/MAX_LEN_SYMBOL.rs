macro_rules! MAX_LEN_SYMBOL {
    () => {
        # [doc = " Maximum symbol value in the length Huffman table (6 bits)"] # [doc = " When this value is decoded, an additional byte is read for extended length"] const MAX_LEN_SYMBOL : u16 = 63 ;
    };
}

MAX_LEN_SYMBOL!();