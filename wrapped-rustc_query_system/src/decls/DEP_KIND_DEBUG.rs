macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! DEP_KIND_DEBUG {
    () => {
        deps!();
        pub static DEP_KIND_DEBUG : AtomicRef < fn (DepKind , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_dep_kind_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
    };
}

DEP_KIND_DEBUG!();