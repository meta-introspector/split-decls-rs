macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl core :: fmt :: Debug for CancellationToken { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("CancellationToken") . field ("is_cancelled" , & self . is_cancelled ()) . finish () } }
    };
}

impl_38!()