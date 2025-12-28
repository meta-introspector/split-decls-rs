macro_rules! deps {
    () => {
        InterestCacheConfig!();
    };
}

macro_rules! configure {
    () => {
        deps!();
        pub (crate) fn configure (new_config : Option < InterestCacheConfig >) { * CONFIG . lock () . unwrap () = new_config . unwrap_or_else (InterestCacheConfig :: disabled) ; INTEREST_CACHE_EPOCH . fetch_add (1 , Ordering :: SeqCst) ; }
    };
}

configure!();