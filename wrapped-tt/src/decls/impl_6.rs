macro_rules! deps {
    () => {
        TtIter!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < S : Copy + fmt :: Debug > fmt :: Debug for TtIter < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TtIter") . field ("remaining" , & self . remaining ()) . finish () } }
    };
}

impl_6!();