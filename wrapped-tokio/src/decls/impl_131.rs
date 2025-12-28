macro_rules! deps {
    () => {
        ReadBuf!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl fmt :: Debug for ReadBuf < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadBuf") . field ("filled" , & self . filled) . field ("initialized" , & self . initialized) . field ("capacity" , & self . capacity ()) . finish () } }
    };
}

impl_131!()