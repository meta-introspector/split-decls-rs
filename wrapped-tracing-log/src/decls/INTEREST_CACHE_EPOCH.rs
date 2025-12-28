macro_rules! INTEREST_CACHE_EPOCH {
    () => {
        static INTEREST_CACHE_EPOCH : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

INTEREST_CACHE_EPOCH!();