macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < C , T > fmt :: Debug for Shared < C , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Shared") . field ("remote" , & self . remote) . field ("prev_sz" , & self . prev_sz) . field ("size" , & self . size) . finish () } }
    };
}

impl_136!()