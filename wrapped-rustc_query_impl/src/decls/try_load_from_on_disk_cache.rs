macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! try_load_from_on_disk_cache {
    () => {
        deps!();
        fn try_load_from_on_disk_cache < 'tcx , Q > (query : Q , tcx : TyCtxt < 'tcx > , dep_node : DepNode) where Q : QueryConfig < QueryCtxt < 'tcx > > , { debug_assert ! (tcx . dep_graph . is_green (& dep_node)) ; let key = Q :: Key :: recover (tcx , & dep_node) . unwrap_or_else (| | { panic ! ("Failed to recover key for {:?} with hash {}" , dep_node , dep_node . hash) }) ; if query . cache_on_disk (tcx , & key) { let _ = query . execute_query (tcx , key) ; } }
    };
}

try_load_from_on_disk_cache!()