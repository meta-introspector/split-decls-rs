macro_rules! deps {
    () => {
        PatStack!();
        PatCx!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > fmt :: Debug for PatStack < 'p , Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "+") ? ; for pat in self . iter () { write ! (f , " {pat:?} +") ? ; } Ok (()) } }
    };
}

impl_106!();