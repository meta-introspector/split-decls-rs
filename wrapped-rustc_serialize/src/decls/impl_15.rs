macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for String { fn encode (& self , s : & mut S) { s . emit_str (& self) ; } }
    };
}

impl_15!();