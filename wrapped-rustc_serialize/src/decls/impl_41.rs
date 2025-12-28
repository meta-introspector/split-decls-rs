macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for path :: PathBuf { fn encode (& self , e : & mut S) { path :: Path :: encode (self , e) ; } }
    };
}

impl_41!();