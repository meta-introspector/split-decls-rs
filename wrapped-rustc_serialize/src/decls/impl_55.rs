macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for VecDeque < T > { fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
    };
}

impl_55!();