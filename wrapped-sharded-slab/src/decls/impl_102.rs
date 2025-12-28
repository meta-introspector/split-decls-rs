macro_rules! deps {
    () => {
        Lifecycle!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < C > fmt :: Debug for Lifecycle < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Lifecycle") . field (& self . state) . finish () } }
    };
}

impl_102!()