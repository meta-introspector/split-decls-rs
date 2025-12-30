// Generated macro for ActiveQueryGuard (struct)
macro_rules! Depcrate_zalsa_localActiveQueryGuard {
() => {
// Module: crate::zalsa_local
// Provides: {"ActiveQueryGuard"}
// Dependencies: {}
# [doc = " When a query is pushed onto the `active_query` stack, this guard"] # [doc = " is returned to represent its slot. The guard can be used to pop"] # [doc = " the query from the stack -- in the case of unwinding, the guard's"] # [doc = " destructor will also remove the query."] pub (crate) struct ActiveQueryGuard < 'me > { local_state : & 'me ZalsaLocal , # [cfg (debug_assertions)] push_len : usize , pub (crate) database_key_index : DatabaseKeyIndex , }
};
}
