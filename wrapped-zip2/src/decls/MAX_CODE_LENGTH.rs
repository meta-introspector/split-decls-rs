macro_rules! MAX_CODE_LENGTH {
    () => {
        # [doc = " Maximum code length in bits for Huffman codes (per ZIP specification)"] const MAX_CODE_LENGTH : usize = 16 ;
    };
}

MAX_CODE_LENGTH!();