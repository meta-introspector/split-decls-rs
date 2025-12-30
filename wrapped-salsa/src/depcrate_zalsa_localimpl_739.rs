// Generated macro for impl_739 (impl)
macro_rules! Depcrate_zalsa_localimpl_739 {
() => {
// Module: crate::zalsa_local
// Provides: {"impl_739"}
// Dependencies: {}
impl Drop for ActiveQueryGuard < '_ > { fn drop (& mut self) { unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { stack . pop (self . database_key_index , # [cfg (debug_assertions)] self . push_len ,) ; }) } ; } }
};
}
