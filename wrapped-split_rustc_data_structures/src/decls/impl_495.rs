macro_rules! deps {
    () => {
        StableOrd!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl StableOrd for Hash128 { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_495!()