macro_rules! deps {
    () => {
        Config!();
        Shard!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < T : fmt :: Debug , C : cfg :: Config > fmt :: Debug for Shard < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("Shard") ; # [cfg (debug_assertions)] d . field ("tid" , & self . tid) ; d . field ("shared" , & self . shared) . finish () } }
    };
}

impl_154!()