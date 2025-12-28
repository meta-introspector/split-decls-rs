macro_rules! deps {
    () => {
        QueryJobId!();
        QueryMap!();
    };
}

macro_rules! pick_query {
    () => {
        deps!();
        fn pick_query < 'a , I : Clone , T , F > (query_map : & QueryMap < I > , queries : & 'a [T] , f : F) -> & 'a T where F : Fn (& T) -> (Span , QueryJobId) , { queries . iter () . min_by_key (| v | { let (span , query) = f (v) ; let hash = query . query (query_map) . hash ; let span_cmp = if span == DUMMY_SP { 1 } else { 0 } ; (span_cmp , hash) }) . unwrap () }
    };
}

pick_query!();