macro_rules! deps {
    () => {
        DepNode!();
        QueryContext!();
        QueryConfig!();
    };
}

macro_rules! ensure_must_run {
    () => {
        deps!();
        # [doc = " Ensure that either this query has all green inputs or been executed."] # [doc = " Executing `query::ensure(D)` is considered a read of the dep-node `D`."] # [doc = " Returns true if the query should still run."] # [doc = ""] # [doc = " This function is particularly useful when executing passes for their"] # [doc = " side-effects -- e.g., in order to report errors for erroneous programs."] # [doc = ""] # [doc = " Note: The optimization is only available during incr. comp."] # [inline (never)] fn ensure_must_run < Q , Qcx > (query : Q , qcx : Qcx , key : & Q :: Key , check_cache : bool ,) -> (bool , Option < DepNode >) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { if query . eval_always () { return (true , None) ; } assert ! (! query . anon ()) ; let dep_node = query . construct_dep_node (* qcx . dep_context () , key) ; let dep_graph = qcx . dep_context () . dep_graph () ; let serialized_dep_node_index = match dep_graph . try_mark_green (qcx , & dep_node) { None => { return (true , Some (dep_node)) ; } Some ((serialized_dep_node_index , dep_node_index)) => { dep_graph . read_index (dep_node_index) ; qcx . dep_context () . profiler () . query_cache_hit (dep_node_index . into ()) ; serialized_dep_node_index } } ; if ! check_cache { return (false , None) ; } let loadable = query . loadable_from_disk (qcx , key , serialized_dep_node_index) ; (! loadable , Some (dep_node)) }
    };
}

ensure_must_run!();