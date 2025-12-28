macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < E : Encoder , K , V , S > Encodable < E > for HashMap < K , V , S > where K : Encodable < E > + Eq , V : Encodable < E > , S : BuildHasher , { fn encode (& self , e : & mut E) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
    };
}

impl_61!();