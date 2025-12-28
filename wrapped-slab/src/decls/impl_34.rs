macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IterMut < '_ , T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("IterMut") . field ("remaining" , & self . len) . finish () } }
    };
}

impl_34!()