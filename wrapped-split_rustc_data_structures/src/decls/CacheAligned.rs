macro_rules! CacheAligned {
    () => {
        # [derive (Default)] # [repr (align (64))] pub struct CacheAligned < T > (pub T) ;
    };
}

CacheAligned!();