macro_rules! interest_cache_epoch {
    () => {
        fn interest_cache_epoch () -> usize { INTEREST_CACHE_EPOCH . load (Ordering :: Relaxed) }
    };
}

interest_cache_epoch!()