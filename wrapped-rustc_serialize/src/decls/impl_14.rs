macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for str { fn encode (& self , s : & mut S) { s . emit_str (self) ; } }
    };
}

impl_14!();