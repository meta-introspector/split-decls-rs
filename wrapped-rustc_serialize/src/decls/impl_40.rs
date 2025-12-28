macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for path :: Path { fn encode (& self , e : & mut S) { self . to_str () . unwrap () . encode (e) ; } }
    };
}

impl_40!()