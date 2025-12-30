// Generated macro for impl_266 (impl)
macro_rules! Depcrate_scopeimpl_266 {
() => {
// Module: crate::scope
// Provides: {"impl_266"}
// Dependencies: {}
impl < 'scope > fmt :: Debug for ScopeFifo < 'scope > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ScopeFifo") . field ("num_fifos" , & self . fifos . len ()) . field ("pool_id" , & self . base . registry . id ()) . field ("panic" , & self . base . panic) . field ("job_completed_latch" , & self . base . job_completed_latch) . finish () } }
};
}
