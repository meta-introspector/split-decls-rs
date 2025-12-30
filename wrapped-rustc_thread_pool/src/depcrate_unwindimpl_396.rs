// Generated macro for impl_396 (impl)
macro_rules! Depcrate_unwindimpl_396 {
() => {
// Module: crate::unwind
// Provides: {"impl_396"}
// Dependencies: {}
impl Drop for AbortIfPanic { fn drop (& mut self) { eprintln ! ("Rayon: detected unexpected panic; aborting") ; :: std :: process :: abort () ; } }
};
}
