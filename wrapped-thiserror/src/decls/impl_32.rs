macro_rules! deps {
    () => {
        Var!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , T : Pointer + ? Sized > Pointer for Var < 'a , T > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Pointer :: fmt (self . 0 , formatter) } }
    };
}

impl_32!();