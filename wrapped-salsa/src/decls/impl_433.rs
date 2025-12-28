macro_rules! deps {
    () => {
        ViewCaster!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ViewCaster { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("DynViewCaster") . field (& self . type_name) . finish () } }
    };
}

impl_433!();