macro_rules! deps {
    () => {
        Config!();
        Shard!();
    };
}

macro_rules! Ptr {
    () => {
        deps!();
        # [derive (Debug)] struct Ptr < T , C : cfg :: Config > (AtomicPtr < alloc :: Track < Shard < T , C > > >) ;
    };
}

Ptr!();