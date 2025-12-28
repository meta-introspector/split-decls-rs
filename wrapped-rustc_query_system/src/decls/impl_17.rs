macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl fmt :: Debug for DepKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* DEP_KIND_DEBUG) (* self , f) } }
    };
}

impl_17!()