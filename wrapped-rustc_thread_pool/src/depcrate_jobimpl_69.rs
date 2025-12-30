// Generated macro for impl_69 (impl)
macro_rules! Depcrate_jobimpl_69 {
() => {
// Module: crate::job
// Provides: {"impl_69"}
// Dependencies: {}
impl < BODY > Job for HeapJob < BODY > where BODY : FnOnce (JobRefId) + Send , { unsafe fn execute (this : * const ()) { let pointer = this . expose_provenance () ; let this = unsafe { Box :: from_raw (this as * mut Self) } ; tlv :: set (this . tlv) ; (this . job) (JobRefId { pointer }) ; } }
};
}
