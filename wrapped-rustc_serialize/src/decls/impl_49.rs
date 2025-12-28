macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < S : Encoder , T : ? Sized + Encodable < S > > Encodable < S > for Box < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
    };
}

impl_49!();