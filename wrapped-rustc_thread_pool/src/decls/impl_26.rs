macro_rules! deps {
    () => {
        BroadcastContext!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for BroadcastContext < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("BroadcastContext") . field ("index" , & self . index ()) . field ("num_threads" , & self . num_threads ()) . field ("pool_id" , & self . worker . registry () . id ()) . finish () } }
    };
}

impl_26!()