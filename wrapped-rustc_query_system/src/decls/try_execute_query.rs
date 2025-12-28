macro_rules! deps {
    () => {
        QueryConfig!();
        QueryJob!();
        QueryResult!();
        Value!();
        DepNode!();
        QueryContext!();
    };
}

macro_rules! try_execute_query {
    () => {
        deps!();
        # [inline (never)] fn try_execute_query < Q , Qcx , const INCR : bool > (query : Q , qcx : Qcx , span : Span , key : Q :: Key , dep_node : Option < DepNode > ,) -> (Q :: Value , Option < DepNodeIndex >) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let state = query . query_state (qcx) ; let key_hash = sharded :: make_hash (& key) ; let mut state_lock = state . active . lock_shard_by_hash (key_hash) ; if qcx . dep_context () . sess () . threads () > 1 { if let Some ((value , index)) = query . query_cache (qcx) . lookup (& key) { qcx . dep_context () . profiler () . query_cache_hit (index . into ()) ; return (value , Some (index)) ; } } let current_job_id = qcx . current_query_job () ; match state_lock . entry (key_hash , equivalent_key (& key) , | (k , _) | sharded :: make_hash (k)) { Entry :: Vacant (entry) => { let id = qcx . next_job_id () ; let job = QueryJob :: new (id , span , current_job_id) ; entry . insert ((key , QueryResult :: Started (job))) ; drop (state_lock) ; execute_job :: < _ , _ , INCR > (query , qcx , state , key , key_hash , id , dep_node) } Entry :: Occupied (mut entry) => { match & mut entry . get_mut () . 1 { QueryResult :: Started (job) => { if sync :: is_dyn_thread_safe () { let latch = job . latch () ; drop (state_lock) ; return wait_for_query (query , qcx , span , key , latch , current_job_id) ; } let id = job . id ; drop (state_lock) ; cycle_error (query , qcx , id , span) } QueryResult :: Poisoned => FatalError . raise () , } } } }
    };
}

try_execute_query!()