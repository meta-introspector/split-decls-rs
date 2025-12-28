macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for NonZero < u32 > { fn encode (& self , s : & mut S) { s . emit_u32 (self . get ()) ; } }
    };
}

impl_12!()