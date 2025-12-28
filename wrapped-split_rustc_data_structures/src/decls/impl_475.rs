macro_rules! deps {
    () => {
        StableOrd!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < T : StableOrd > StableOrd for & T { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_475!()