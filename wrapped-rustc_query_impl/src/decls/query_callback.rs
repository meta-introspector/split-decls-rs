macro_rules! deps {
    () => {
        QueryConfigRestored!();
        QueryCtxt!();
    };
}

macro_rules! query_callback {
    () => {
        deps!();
        pub (crate) fn query_callback < 'tcx , Q > (is_anon : bool , is_eval_always : bool) -> DepKindStruct < 'tcx > where Q : QueryConfigRestored < 'tcx > , { let fingerprint_style = < Q :: Config as QueryConfig < QueryCtxt < 'tcx > > > :: Key :: fingerprint_style () ; if is_anon || ! fingerprint_style . reconstructible () { return DepKindStruct { is_anon , is_eval_always , fingerprint_style , force_from_dep_node : None , try_load_from_on_disk_cache : None , name : Q :: NAME , } ; } DepKindStruct { is_anon , is_eval_always , fingerprint_style , force_from_dep_node : Some (| tcx , dep_node , _ | { force_from_dep_node (Q :: config (tcx) , tcx , dep_node) }) , try_load_from_on_disk_cache : Some (| tcx , dep_node | { try_load_from_on_disk_cache (Q :: config (tcx) , tcx , dep_node) }) , name : Q :: NAME , } }
    };
}

query_callback!();