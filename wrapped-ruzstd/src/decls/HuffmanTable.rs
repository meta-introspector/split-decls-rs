macro_rules! HuffmanTable {
    () => {
        pub struct HuffmanTable { # [doc = " Index is the symbol, values are the bitstring in the lower bits of the u32 and the amount of bits in the u8"] codes : Vec < (u32 , u8) > , }
    };
}

HuffmanTable!();