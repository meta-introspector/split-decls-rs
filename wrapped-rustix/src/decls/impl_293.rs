macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl fmt :: Display for Errno { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { std :: io :: Error :: from (* self) . fmt (f) } # [cfg (not (feature = "std"))] { write ! (f , "os error {}" , self . raw_os_error ()) } } }
    };
}

impl_293!()