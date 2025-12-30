// Generated macro for impl_23 (impl)
macro_rules! Depcrate_attrimpl_23 {
() => {
// Module: crate::attr
// Provides: {"impl_23"}
// Dependencies: {}
impl Drop for Context { fn drop (& mut self) { if ! thread :: panicking () && self . error . borrow () . is_some () { panic ! ("context need to be checked") ; } } }
};
}
