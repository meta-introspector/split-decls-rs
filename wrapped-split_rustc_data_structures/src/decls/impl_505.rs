macro_rules! deps {
    () => {
        StableOrd!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < T1 : StableOrd , T2 : StableOrd > StableOrd for (T1 , T2) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_505!();