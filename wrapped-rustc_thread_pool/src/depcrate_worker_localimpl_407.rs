// Generated macro for impl_407 (impl)
macro_rules! Depcrate_worker_localimpl_407 {
() => {
// Module: crate::worker_local
// Provides: {"impl_407"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for WorkerLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WorkerLocal") . field ("registry" , & self . registry . id ()) . finish () } }
};
}
