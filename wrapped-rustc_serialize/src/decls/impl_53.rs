macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for ThinVec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
    };
}

impl_53!();