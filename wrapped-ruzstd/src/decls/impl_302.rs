macro_rules! deps {
    () => {
        FSEDecoder!();
        Entry!();
        FSEDecoderError!();
        FSETable!();
        State!();
        BitReaderReversed!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 't > FSEDecoder < 't > { # [doc = " Initialize a new Finite State Entropy decoder."] pub fn new (table : & 't FSETable) -> FSEDecoder < 't > { FSEDecoder { state : table . decode . first () . copied () . unwrap_or (Entry { base_line : 0 , num_bits : 0 , symbol : 0 , }) , table , } } # [doc = " Returns the byte associated with the symbol the internal cursor is pointing at."] pub fn decode_symbol (& self) -> u8 { self . state . symbol } # [doc = " Initialize internal state and prepare for decoding. After this, `decode_symbol` can be called"] # [doc = " to read the first symbol and `update_state` can be called to prepare to read the next symbol."] pub fn init_state (& mut self , bits : & mut BitReaderReversed < '_ >) -> Result < () , FSEDecoderError > { if self . table . accuracy_log == 0 { return Err (FSEDecoderError :: TableIsUninitialized) ; } let new_state = bits . get_bits (self . table . accuracy_log) ; self . state = self . table . decode [new_state as usize] ; Ok (()) } # [doc = " Advance the internal state to decode the next symbol in the bitstream."] pub fn update_state (& mut self , bits : & mut BitReaderReversed < '_ >) { let num_bits = self . state . num_bits ; let add = bits . get_bits (num_bits) ; let base_line = self . state . base_line ; let new_state = base_line + add as u32 ; self . state = self . table . decode [new_state as usize] ; } }
    };
}

impl_302!();