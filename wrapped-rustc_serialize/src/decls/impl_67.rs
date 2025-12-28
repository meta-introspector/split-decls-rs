macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < E : Encoder , T , S > Encodable < E > for indexmap :: IndexSet < T , S > where T : Encodable < E > + Hash + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
    };
}

impl_67!();