// Generated macro for THREAD_ID_MANAGER (static)
macro_rules! Depcrate_thread_idTHREAD_ID_MANAGER {
() => {
// Module: crate::thread_id
// Provides: {"THREAD_ID_MANAGER"}
// Dependencies: {}
static THREAD_ID_MANAGER : Mutex < ThreadIdManager > = Mutex :: new (ThreadIdManager :: new ()) ;
};
}
