macro_rules! deps {
    () => {
        TryInitError!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl fmt :: Debug for TryInitError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { fmt :: Debug :: fmt (& self . inner , f) } # [cfg (not (feature = "std"))] { f . write_str ("TryInitError(())") } } }
    };
}

impl_150!()