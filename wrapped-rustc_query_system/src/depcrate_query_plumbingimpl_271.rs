// Generated macro for impl_271 (impl)
macro_rules! Depcrate_query_plumbingimpl_271 {
() => {
// Module: crate::query::plumbing
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'tcx , K , I > JobOwner < 'tcx , K , I > where K : Eq + Hash + Copy , { # [doc = " Completes the query by updating the query cache with the `result`,"] # [doc = " signals the waiter and forgets the JobOwner, so it won't poison the query"] fn complete < C > (self , cache : & C , key_hash : u64 , result : C :: Value , dep_node_index : DepNodeIndex) where C : QueryCache < Key = K > , { let key = self . key ; let state = self . state ; mem :: forget (self) ; cache . complete (key , result , dep_node_index) ; let job = { let mut shard = state . active . lock_shard_by_hash (key_hash) ; match shard . find_entry (key_hash , equivalent_key (& key)) { Err (_) => None , Ok (occupied) => Some (occupied . remove () . 0 . 1) , } } ; let job = job . expect ("active query job entry") . expect_job () ; job . signal_complete () ; } }
};
}
