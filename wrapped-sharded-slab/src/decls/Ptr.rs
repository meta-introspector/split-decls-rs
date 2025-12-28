macro_rules! deps {
    () => {
        Shard!();
        Config!();
    };
}

macro_rules! Ptr {
    () => {
        deps!();
        # [derive (Debug)] struct Ptr < T , C : cfg :: Config > (AtomicPtr < alloc :: Track < Shard < T , C > > >) ;
    };
}

Ptr!()