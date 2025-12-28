macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Debug for Sha1Core { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha1Core { ... }") } }
    };
}

impl_12!();