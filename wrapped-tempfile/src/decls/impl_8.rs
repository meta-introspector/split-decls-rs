macro_rules! deps {
    () => {
        TempDir!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl fmt :: Debug for TempDir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TempDir") . field ("path" , & self . path ()) . finish () } }
    };
}

impl_8!()