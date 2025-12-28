macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for Cow < '_ , str > { fn encode (& self , s : & mut S) { let val : & str = self ; val . encode (s) } }
    };
}

impl_31!()