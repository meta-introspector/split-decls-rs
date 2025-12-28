macro_rules! deps {
    () => {
        Document!();
        Deserializer!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < S > From < crate :: Document < S > > for Deserializer < S > { fn from (doc : crate :: Document < S >) -> Self { let crate :: Document { root , raw , .. } = doc ; let raw = Some (raw) ; Self { root , raw } } }
    };
}

impl_330!()