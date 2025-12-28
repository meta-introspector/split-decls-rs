macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl VariableOutputCore for Sha256VarCore { const TRUNC_SIDE : TruncSide = TruncSide :: Left ; # [inline] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > { let state = match output_size { 28 => consts :: H256_224 , 32 => consts :: H256_256 , _ => return Err (InvalidOutputSize) , } ; let block_len = 0 ; Ok (Self { state , block_len }) } # [inline] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bs = Self :: BlockSize :: U64 ; let bit_len = 8 * (buffer . get_pos () as u64 + bs * self . block_len) ; buffer . len64_padding_be (bit_len , | b | compress256 (& mut self . state , & [b . 0])) ; for (chunk , v) in out . chunks_exact_mut (4) . zip (self . state . iter ()) { chunk . copy_from_slice (& v . to_be_bytes ()) ; } } }
    };
}

impl_7!()