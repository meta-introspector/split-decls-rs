macro_rules! deps {
    () => {
        QueryContext!();
        Value!();
        QueryJobId!();
        QueryConfig!();
    };
}

macro_rules! execute_job_non_incr {
    () => {
        deps!();
        # [inline (always)] fn execute_job_non_incr < Q , Qcx > (query : Q , qcx : Qcx , key : Q :: Key , job_id : QueryJobId ,) -> (Q :: Value , DepNodeIndex) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { debug_assert ! (! qcx . dep_context () . dep_graph () . is_fully_enabled ()) ; if cfg ! (debug_assertions) { let _ = key . to_fingerprint (* qcx . dep_context ()) ; } let prof_timer = qcx . dep_context () . profiler () . query_provider () ; let result = qcx . start_query (job_id , query . depth_limit () , | | query . compute (qcx , key)) ; let dep_node_index = qcx . dep_context () . dep_graph () . next_virtual_depnode_index () ; prof_timer . finish_with_query_invocation_id (dep_node_index . into ()) ; if cfg ! (debug_assertions) && let Some (hash_result) = query . hash_result () { qcx . dep_context () . with_stable_hashing_context (| mut hcx | { hash_result (& mut hcx , & result) ; }) ; } (result , dep_node_index) }
    };
}

execute_job_non_incr!()