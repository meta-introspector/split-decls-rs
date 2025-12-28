macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! DEF_ID_DEBUG {
    () => {
        deps!();
        pub static DEF_ID_DEBUG : AtomicRef < fn (DefId , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_def_id_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
    };
}

DEF_ID_DEBUG!()