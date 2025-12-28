macro_rules! deps {
    () => {
        Result!();
        EcPrivateKey!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl fmt :: Debug for EcPrivateKey < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("EcPrivateKey") . field ("parameters" , & self . parameters) . field ("public_key" , & self . public_key) . finish_non_exhaustive () } }
    };
}

impl_55!();