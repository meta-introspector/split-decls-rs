macro_rules! deps {
    () => {
        QueryCtxt!();
        QueryConfigRestored!();
    };
}

macro_rules! encode_query_results {
    () => {
        deps!();
        pub (crate) fn encode_query_results < 'a , 'tcx , Q > (query : Q :: Config , qcx : QueryCtxt < 'tcx > , encoder : & mut CacheEncoder < 'a , 'tcx > , query_result_index : & mut EncodedDepNodeIndex ,) where Q : super :: QueryConfigRestored < 'tcx > , Q :: RestoredValue : Encodable < CacheEncoder < 'a , 'tcx > > , { let _timer = qcx . profiler () . generic_activity_with_arg ("encode_query_results_for" , query . name ()) ; assert ! (query . query_state (qcx) . all_inactive ()) ; let cache = query . query_cache (qcx) ; cache . iter (& mut | key , value , dep_node | { if query . cache_on_disk (qcx . tcx , key) { let dep_node = SerializedDepNodeIndex :: new (dep_node . index ()) ; query_result_index . push ((dep_node , AbsoluteBytePos :: new (encoder . position ()))) ; encoder . encode_tagged (dep_node , & Q :: restore (* value)) ; } }) ; }
    };
}

encode_query_results!()