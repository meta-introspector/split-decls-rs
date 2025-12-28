macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for Hash128 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u128 () . to_le_bytes ()) ; } }
    };
}

impl_74!()