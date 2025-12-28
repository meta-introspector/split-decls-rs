macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < S : Encoder , K , V > Encodable < S > for BTreeMap < K , V > where K : Encodable < S > + PartialEq + Ord , V : Encodable < S > , { fn encode (& self , e : & mut S) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
    };
}

impl_57!()