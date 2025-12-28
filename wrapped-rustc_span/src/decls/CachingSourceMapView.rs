macro_rules! deps {
    () => {
        SourceMap!();
        CacheEntry!();
    };
}

macro_rules! CachingSourceMapView {
    () => {
        deps!();
        # [derive (Clone)] pub struct CachingSourceMapView < 'sm > { source_map : & 'sm SourceMap , line_cache : [CacheEntry ; 3] , time_stamp : usize , }
    };
}

CachingSourceMapView!();