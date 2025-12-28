macro_rules! deps {
    () => {
        BarrierWaitResult!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl fmt :: Debug for BarrierWaitResult { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BarrierWaitResult") . field ("is_leader" , & self . is_leader ()) . finish () } }
    };
}

impl_208!();