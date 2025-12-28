macro_rules! deps {
    () => {
        BitReaderReversed!();
        HuffmanTable!();
        HuffmanDecoder!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < 't > HuffmanDecoder < 't > { # [doc = " Create a new decoder with the provided table"] pub fn new (table : & 't HuffmanTable) -> HuffmanDecoder < 't > { HuffmanDecoder { table , state : 0 } } # [doc = " Decode the symbol the internal state (cursor) is pointed at and return the"] # [doc = " decoded literal."] pub fn decode_symbol (& mut self) -> u8 { self . table . decode [self . state as usize] . symbol } # [doc = " Initialize internal state and prepare to decode data. Then, `decode_symbol` can be called"] # [doc = " to read the byte the internal cursor is pointing at, and `next_state` can be called to advance"] # [doc = " the cursor until the max number of bits has been read."] pub fn init_state (& mut self , br : & mut BitReaderReversed < '_ >) -> u8 { let num_bits = self . table . max_num_bits ; let new_bits = br . get_bits (num_bits) ; self . state = new_bits ; num_bits } # [doc = " Advance the internal cursor to the next symbol. After this, you can call `decode_symbol`"] # [doc = " to read from the new position."] pub fn next_state (& mut self , br : & mut BitReaderReversed < '_ >) -> u8 { let num_bits = self . table . decode [self . state as usize] . num_bits ; let new_bits = br . get_bits (num_bits) ; self . state <<= num_bits ; self . state &= self . table . decode . len () as u64 - 1 ; self . state |= new_bits ; num_bits } }
    };
}

impl_337!()