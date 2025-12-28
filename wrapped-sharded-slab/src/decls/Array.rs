macro_rules! deps {
    () => {
        Ptr!();
        Config!();
    };
}

macro_rules! Array {
    () => {
        deps!();
        pub (crate) struct Array < T , C : cfg :: Config > { shards : Box < [Ptr < T , C >] > , max : AtomicUsize , }
    };
}

Array!()