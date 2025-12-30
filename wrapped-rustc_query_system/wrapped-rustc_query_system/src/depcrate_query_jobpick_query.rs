// Generated macro for pick_query (function)
macro_rules! Depcrate_query_jobpick_query {
() => {
// Module: crate::query::job
// Provides: {"pick_query"}
// Dependencies: {}
fn pick_query < 'a , I : Clone , T , F > (query_map : & QueryMap < I > , queries : & 'a [T] , f : F) -> & 'a T where F : Fn (& T) -> (Span , QueryJobId) , { queries . iter () . min_by_key (| v | { let (span , query) = f (v) ; let hash = query . query (query_map) . hash ; let span_cmp = if span == DUMMY_SP { 1 } else { 0 } ; (span_cmp , hash) }) . unwrap () }
};
}
