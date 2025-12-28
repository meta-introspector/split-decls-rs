macro_rules! deps {
    () => {
        KnownLayout!();
        MaybeUninit!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl < T : ? Sized + KnownLayout > fmt :: Debug for MaybeUninit < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (core :: any :: type_name :: < Self > ()) } }
    };
}

impl_424!()