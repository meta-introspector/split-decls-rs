macro_rules! deps {
    () => {
        ReusableBoxFuture!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T > fmt :: Debug for ReusableBoxFuture < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReusableBoxFuture") . finish () } }
    };
}

impl_81!();