macro_rules! try_load_from_disk {
    () => {
        pub (crate) fn try_load_from_disk < 'tcx , V > (tcx : TyCtxt < 'tcx > , prev_index : SerializedDepNodeIndex , index : DepNodeIndex ,) -> Option < V > where V : for < 'a > Decodable < CacheDecoder < 'a , 'tcx > > , { let on_disk_cache = tcx . query_system . on_disk_cache . as_ref () ? ; let prof_timer = tcx . prof . incr_cache_loading () ; let value = tcx . dep_graph . with_query_deserialization (| | on_disk_cache . try_load_query_result (tcx , prev_index)) ; prof_timer . finish_with_query_invocation_id (index . into ()) ; value }
    };
}

try_load_from_disk!();