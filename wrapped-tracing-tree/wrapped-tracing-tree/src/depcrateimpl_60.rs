// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl Drop for RecursiveGuard { fn drop (& mut self) { self . 0 . with (| is_empty | is_empty . store (true , Ordering :: Relaxed)) ; } }
};
}
