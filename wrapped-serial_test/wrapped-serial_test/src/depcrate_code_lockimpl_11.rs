// Generated macro for impl_11 (impl)
macro_rules! Depcrate_code_lockimpl_11 {
() => {
// Module: crate::code_lock
// Provides: {"impl_11"}
// Dependencies: {}
impl UniqueReentrantMutex { fn new_mutex (name : & str) -> Self { Self { locks : Locks :: new (name) , id : MUTEX_ID . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst) , } } }
};
}
