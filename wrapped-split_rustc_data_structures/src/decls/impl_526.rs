macro_rules! deps {
    () => {
        StableOrd!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl StableOrd for bool { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_526!()