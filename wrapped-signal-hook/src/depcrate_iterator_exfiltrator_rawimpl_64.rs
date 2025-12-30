// Generated macro for impl_64 (impl)
macro_rules! Depcrate_iterator_exfiltrator_rawimpl_64 {
() => {
// Module: crate::iterator::exfiltrator::raw
// Provides: {"impl_64"}
// Dependencies: {}
impl Drop for Slot { fn drop (& mut self) { let ptr = self . 0 . load (Ordering :: Acquire) ; if ! ptr . is_null () { drop (unsafe { Box :: from_raw (ptr) }) ; } } }
};
}
