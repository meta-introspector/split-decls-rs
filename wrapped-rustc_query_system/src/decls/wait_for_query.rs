macro_rules! deps {
    () => {
        QueryResult!();
        QueryLatch!();
        QueryContext!();
        QueryJobId!();
        QueryConfig!();
        Value!();
        QueryInfo!();
    };
}

macro_rules! wait_for_query {
    () => {
        deps!();
        # [inline (always)] fn wait_for_query < Q , Qcx > (query : Q , qcx : Qcx , span : Span , key : Q :: Key , latch : QueryLatch < Qcx :: QueryInfo > , current : Option < QueryJobId > ,) -> (Q :: Value , Option < DepNodeIndex >) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let query_blocked_prof_timer = qcx . dep_context () . profiler () . query_blocked () ; let result = latch . wait_on (qcx , current , span) ; match result { Ok (()) => { let Some ((v , index)) = query . query_cache (qcx) . lookup (& key) else { outline (| | { let key_hash = sharded :: make_hash (& key) ; let shard = query . query_state (qcx) . active . lock_shard_by_hash (key_hash) ; match shard . find (key_hash , equivalent_key (& key)) { Some ((_ , QueryResult :: Poisoned)) => FatalError . raise () , _ => panic ! ("query '{}' result must be in the cache or the query must be poisoned after a wait" , query . name ()) , } }) } ; qcx . dep_context () . profiler () . query_cache_hit (index . into ()) ; query_blocked_prof_timer . finish_with_query_invocation_id (index . into ()) ; (v , Some (index)) } Err (cycle) => (mk_cycle (query , qcx , cycle . lift (qcx)) , None) , } }
    };
}

wait_for_query!();