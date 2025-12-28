macro_rules! deps {
    () => {
        SetGlobalDefaultError!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl fmt :: Debug for SetGlobalDefaultError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("SetGlobalDefaultError") . field (& Self :: MESSAGE) . finish () } }
    };
}

impl_76!()