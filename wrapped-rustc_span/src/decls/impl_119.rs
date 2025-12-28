macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl fmt :: Debug for LocalDefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . to_def_id () . fmt (f) } }
    };
}

impl_119!()