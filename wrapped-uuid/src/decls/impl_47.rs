macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl fmt :: Display for Variant { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Variant :: NCS => write ! (f , "NCS") , Variant :: RFC4122 => write ! (f , "RFC4122") , Variant :: Microsoft => write ! (f , "Microsoft") , Variant :: Future => write ! (f , "Future") , } } }
    };
}

impl_47!();