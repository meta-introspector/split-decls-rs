macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for Cow < '_ , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn encode (& self , s : & mut S) { let slice : & [T] = self ; slice . encode (s) ; } }
    };
}

impl_29!();