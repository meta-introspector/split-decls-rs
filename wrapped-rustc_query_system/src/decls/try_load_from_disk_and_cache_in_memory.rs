macro_rules! deps {
    () => {
        QueryContext!();
        DepNode!();
        DepGraphData!();
        QueryConfig!();
        Deps!();
        Value!();
    };
}

macro_rules! try_load_from_disk_and_cache_in_memory {
    () => {
        deps!();
        # [inline (always)] fn try_load_from_disk_and_cache_in_memory < Q , Qcx > (query : Q , dep_graph_data : & DepGraphData < Qcx :: Deps > , qcx : Qcx , key : & Q :: Key , dep_node : & DepNode ,) -> Option < (Q :: Value , DepNodeIndex) > where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let (prev_dep_node_index , dep_node_index) = dep_graph_data . try_mark_green (qcx , dep_node) ? ; debug_assert ! (dep_graph_data . is_index_green (prev_dep_node_index)) ; if let Some (result) = query . try_load_from_disk (qcx , key , prev_dep_node_index , dep_node_index) { if std :: intrinsics :: unlikely (qcx . dep_context () . sess () . opts . unstable_opts . query_dep_graph) { dep_graph_data . mark_debug_loaded_from_disk (* dep_node) } let prev_fingerprint = dep_graph_data . prev_fingerprint_of (prev_dep_node_index) ; let try_verify = prev_fingerprint . split () . 1 . as_u64 () . is_multiple_of (32) ; if std :: intrinsics :: unlikely (try_verify || qcx . dep_context () . sess () . opts . unstable_opts . incremental_verify_ich ,) { incremental_verify_ich (* qcx . dep_context () , dep_graph_data , & result , prev_dep_node_index , query . hash_result () , query . format_value () ,) ; } return Some ((result , dep_node_index)) ; } debug_assert ! (! query . cache_on_disk (* qcx . dep_context () , key) || ! qcx . dep_context () . fingerprint_style (dep_node . kind) . reconstructible () , "missing on-disk cache entry for {dep_node:?}") ; debug_assert ! (! query . loadable_from_disk (qcx , key , prev_dep_node_index) , "missing on-disk cache entry for loadable {dep_node:?}") ; let prof_timer = qcx . dep_context () . profiler () . query_provider () ; let result = qcx . dep_context () . dep_graph () . with_ignore (| | query . compute (qcx , * key)) ; prof_timer . finish_with_query_invocation_id (dep_node_index . into ()) ; incremental_verify_ich (* qcx . dep_context () , dep_graph_data , & result , prev_dep_node_index , query . hash_result () , query . format_value () ,) ; Some ((result , dep_node_index)) }
    };
}

try_load_from_disk_and_cache_in_memory!();