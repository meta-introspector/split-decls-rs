macro_rules! deps {
    () => {
        Deserializer!();
        DocumentMut!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl From < crate :: DocumentMut > for Deserializer { fn from (doc : crate :: DocumentMut) -> Self { let crate :: DocumentMut { root , .. } = doc ; Self { root , raw : None } } }
    };
}

impl_329!();