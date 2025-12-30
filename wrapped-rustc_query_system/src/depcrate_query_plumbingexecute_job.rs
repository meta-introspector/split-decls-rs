// Generated macro for execute_job (function)
macro_rules! Depcrate_query_plumbingexecute_job {
() => {
// Module: crate::query::plumbing
// Provides: {"execute_job"}
// Dependencies: {}
# [inline (always)] fn execute_job < Q , Qcx , const INCR : bool > (query : Q , qcx : Qcx , state : & QueryState < Q :: Key , Qcx :: QueryInfo > , key : Q :: Key , key_hash : u64 , id : QueryJobId , dep_node : Option < DepNode > ,) -> (Q :: Value , Option < DepNodeIndex >) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let job_owner = JobOwner { state , key } ; debug_assert_eq ! (qcx . dep_context () . dep_graph () . is_fully_enabled () , INCR) ; let (result , dep_node_index) = if INCR { execute_job_incr (query , qcx , qcx . dep_context () . dep_graph () . data () . unwrap () , key , dep_node , id ,) } else { execute_job_non_incr (query , qcx , key , id) } ; let cache = query . query_cache (qcx) ; if query . feedable () { if let Some ((cached_result , _)) = cache . lookup (& key) { let Some (hasher) = query . hash_result () else { panic ! ("no_hash fed query later has its value computed.\n\
                    Remove `no_hash` modifier to allow recomputation.\n\
                    The already cached value: {}" , (query . format_value ()) (& cached_result)) ; } ; let (old_hash , new_hash) = qcx . dep_context () . with_stable_hashing_context (| mut hcx | { (hasher (& mut hcx , & cached_result) , hasher (& mut hcx , & result)) }) ; let formatter = query . format_value () ; if old_hash != new_hash { assert ! (qcx . dep_context () . sess () . dcx () . has_errors () . is_some () , "Computed query value for {:?}({:?}) is inconsistent with fed value,\n\
                        computed={:#?}\nfed={:#?}" , query . dep_kind () , key , formatter (& result) , formatter (& cached_result) ,) ; } } } job_owner . complete (cache , key_hash , result , dep_node_index) ; (result , Some (dep_node_index)) }
};
}
