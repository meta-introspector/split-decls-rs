macro_rules! deps {
    () => {
        DepNode!();
    };
}

macro_rules! DEP_NODE_DEBUG {
    () => {
        deps!();
        pub static DEP_NODE_DEBUG : AtomicRef < fn (DepNode , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_dep_node_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
    };
}

DEP_NODE_DEBUG!();