// Generated macro for impl_72 (impl)
macro_rules! Depcrate_jobimpl_72 {
() => {
// Module: crate::job
// Provides: {"impl_72"}
// Dependencies: {}
impl < BODY > Job for ArcJob < BODY > where BODY : Fn (JobRefId) + Send + Sync , { unsafe fn execute (this : * const ()) { let pointer = this . expose_provenance () ; let this = unsafe { Arc :: from_raw (this as * mut Self) } ; (this . job) (JobRefId { pointer }) ; } }
};
}
