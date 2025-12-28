macro_rules! deps {
    () => {
        SharedNext!();
        Next!();
    };
}

macro_rules! macro_423 {
    () => {
        deps!();
        lazy_static ! { static ref NEXT_GLOBAL : SharedNext = SharedNext (Arc :: new (Next (AtomicUsize :: new (1)))) ; }
    };
}

macro_423!()