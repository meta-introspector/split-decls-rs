// Generated macro for impl_491 (impl)
macro_rules! Depcrate_internerimpl_491 {
() => {
// Module: crate::interner
// Provides: {"impl_491"}
// Dependencies: {}
impl < I : Interner > search_graph :: Cx for I { type Input = CanonicalInput < I > ; type Result = QueryResult < I > ; type DepNodeIndex = I :: DepNodeIndex ; type Tracked < T : Debug + Clone > = I :: Tracked < T > ; fn mk_tracked < T : Debug + Clone > (self , data : T , dep_node_index : I :: DepNodeIndex ,) -> I :: Tracked < T > { I :: mk_tracked (self , data , dep_node_index) } fn get_tracked < T : Debug + Clone > (self , tracked : & I :: Tracked < T >) -> T { I :: get_tracked (self , tracked) } fn with_cached_task < T > (self , task : impl FnOnce () -> T) -> (T , I :: DepNodeIndex) { I :: with_cached_task (self , task) } fn with_global_cache < R > (self , f : impl FnOnce (& mut search_graph :: GlobalCache < Self >) -> R) -> R { I :: with_global_cache (self , f) } fn evaluation_is_concurrent (& self) -> bool { self . evaluation_is_concurrent () } }
};
}
