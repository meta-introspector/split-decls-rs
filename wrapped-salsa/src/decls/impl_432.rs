macro_rules! deps {
    () => {
        Views!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Views { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Views") . field ("view_casters" , & self . view_casters) . finish () } }
    };
}

impl_432!();