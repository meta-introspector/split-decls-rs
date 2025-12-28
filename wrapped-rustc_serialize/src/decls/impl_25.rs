macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for Vec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
    };
}

impl_25!()