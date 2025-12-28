macro_rules! deps {
    () => {
        Encodable!();
        Encoder!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < E : Encoder , T : Encodable < E > > Encodable < E > for Rc < [T] > { fn encode (& self , s : & mut E) { let slice : & [T] = self ; slice . encode (s) ; } }
    };
}

impl_69!()