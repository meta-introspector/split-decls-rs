macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for () { fn encode (& self , _s : & mut S) { } }
    };
}

impl_17!();