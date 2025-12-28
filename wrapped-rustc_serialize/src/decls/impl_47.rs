macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for Arc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
    };
}

impl_47!()