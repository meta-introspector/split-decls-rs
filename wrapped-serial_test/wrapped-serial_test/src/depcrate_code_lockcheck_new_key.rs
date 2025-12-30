// Generated macro for check_new_key (function)
macro_rules! Depcrate_code_lockcheck_new_key {
() => {
// Module: crate::code_lock
// Provides: {"check_new_key"}
// Dependencies: {}
pub (crate) fn check_new_key (name : & str) { if global_locks () . contains (name) { return ; } ; let entry = global_locks () . entry (name . to_owned ()) ; match entry { Entry :: Occupied (o) => o , Entry :: Vacant (v) => v . insert_entry (UniqueReentrantMutex :: new_mutex (name)) , } ; }
};
}
