macro_rules! CacheAligned {
    () => {
        # [repr (align (64))] # [derive (Debug)] struct CacheAligned < T > (T) ;
    };
}

CacheAligned!();