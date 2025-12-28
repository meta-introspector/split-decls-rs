macro_rules! deps {
    () => {
        Clear!();
        Pool!();
        Config!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for Pool < T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pool") . field ("shards" , & self . shards) . field ("config" , & C :: debug ()) . finish () } }
    };
}

impl_16!()