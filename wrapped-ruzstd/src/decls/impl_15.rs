macro_rules! deps {
    () => {
        BitWriter!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl BitWriter < Vec < u8 > > { # [doc = " Initialize a new writer."] pub fn new () -> Self { Self { output : Vec :: new () , partial : 0 , bits_in_partial : 0 , bit_idx : 0 , } } }
    };
}

impl_15!()