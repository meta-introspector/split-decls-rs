macro_rules! deps {
    () => {
        Config!();
        Slab!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < T : fmt :: Debug , C : cfg :: Config > fmt :: Debug for Slab < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Slab") . field ("shards" , & self . shards) . field ("config" , & C :: debug ()) . finish () } }
    };
}

impl_185!();