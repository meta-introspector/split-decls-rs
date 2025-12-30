// Generated macro for impl_137 (impl)
macro_rules! Depcrate_internals_ctxtimpl_137 {
() => {
// Module: crate::internals::ctxt
// Provides: {"impl_137"}
// Dependencies: {}
impl Drop for Ctxt { fn drop (& mut self) { if ! thread :: panicking () && self . errors . borrow () . is_some () { panic ! ("forgot to check for errors") ; } } }
};
}
