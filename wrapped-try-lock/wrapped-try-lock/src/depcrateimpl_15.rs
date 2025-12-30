// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a , T > Drop for Locked < 'a , T > { # [inline] fn drop (& mut self) { self . lock . is_locked . store (false , self . order) ; } }
};
}
