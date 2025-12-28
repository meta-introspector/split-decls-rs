macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for Rc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
    };
}

impl_22!();