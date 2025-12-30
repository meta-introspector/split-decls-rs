// Generated macro for impl_736 (impl)
macro_rules! Depcrate_zalsa_localimpl_736 {
() => {
// Module: crate::zalsa_local
// Provides: {"impl_736"}
// Dependencies: {}
impl Drop for ActiveQueryGuard < '_ > { fn drop (& mut self) { unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { stack . pop (self . database_key_index , # [cfg (debug_assertions)] self . push_len ,) ; }) } ; } }
};
}
