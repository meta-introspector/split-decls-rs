macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < S : Encoder , T > Encodable < S > for BTreeSet < T > where T : Encodable < S > + PartialEq + Ord , { fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
    };
}

impl_59!();