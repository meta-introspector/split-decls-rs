macro_rules! deps {
    () => {
        JoinHandle!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T > fmt :: Debug for JoinHandle < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("JoinHandle { .. }") } }
    };
}

impl_54!()