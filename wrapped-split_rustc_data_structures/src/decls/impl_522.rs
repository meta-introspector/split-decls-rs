macro_rules! deps {
    () => {
        StableOrd!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        impl StableOrd for String { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_522!()