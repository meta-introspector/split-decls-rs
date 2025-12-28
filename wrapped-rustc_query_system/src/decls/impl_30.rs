macro_rules! deps {
    () => {
        WorkProductId!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl StableOrd for WorkProductId { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_30!();