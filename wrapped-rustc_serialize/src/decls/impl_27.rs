macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > , const N : usize > Encodable < S > for [T ; N] { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
    };
}

impl_27!()