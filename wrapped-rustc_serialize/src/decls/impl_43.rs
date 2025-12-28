macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > + Copy > Encodable < S > for Cell < T > { fn encode (& self , s : & mut S) { self . get () . encode (s) ; } }
    };
}

impl_43!()