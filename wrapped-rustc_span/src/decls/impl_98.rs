macro_rules! deps {
    () => {
        DefPathHash!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl StableOrd for DefPathHash { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_98!()