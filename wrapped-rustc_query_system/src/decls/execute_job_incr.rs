macro_rules! deps {
    () => {
        QueryConfig!();
        Value!();
        DepNode!();
        Deps!();
        DepGraphData!();
        QueryContext!();
        QueryJobId!();
    };
}

macro_rules! execute_job_incr {
    () => {
        deps!();
        # [inline (always)] fn execute_job_incr < Q , Qcx > (query : Q , qcx : Qcx , dep_graph_data : & DepGraphData < Qcx :: Deps > , key : Q :: Key , mut dep_node_opt : Option < DepNode > , job_id : QueryJobId ,) -> (Q :: Value , DepNodeIndex) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { if ! query . anon () && ! query . eval_always () { let dep_node = dep_node_opt . get_or_insert_with (| | query . construct_dep_node (* qcx . dep_context () , & key)) ; if let Some (ret) = qcx . start_query (job_id , false , | | { try_load_from_disk_and_cache_in_memory (query , dep_graph_data , qcx , & key , dep_node) }) { return ret ; } } let prof_timer = qcx . dep_context () . profiler () . query_provider () ; let (result , dep_node_index) = qcx . start_query (job_id , query . depth_limit () , | | { if query . anon () { return dep_graph_data . with_anon_task_inner (* qcx . dep_context () , query . dep_kind () , | | query . compute (qcx , key) ,) ; } let dep_node = dep_node_opt . unwrap_or_else (| | query . construct_dep_node (* qcx . dep_context () , & key)) ; dep_graph_data . with_task (dep_node , (qcx , query) , key , | (qcx , query) , key | query . compute (qcx , key) , query . hash_result () ,) }) ; prof_timer . finish_with_query_invocation_id (dep_node_index . into ()) ; (result , dep_node_index) }
    };
}

execute_job_incr!();