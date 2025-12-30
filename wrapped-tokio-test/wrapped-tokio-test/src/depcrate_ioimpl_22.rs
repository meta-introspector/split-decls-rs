// Generated macro for impl_22 (impl)
macro_rules! Depcrate_ioimpl_22 {
() => {
// Module: crate::io
// Provides: {"impl_22"}
// Dependencies: {}
impl Mock { fn maybe_wakeup_reader (& mut self) { match self . inner . action () { Some (& mut Action :: Read (_)) | Some (& mut Action :: ReadError (_)) | None => { if let Some (waker) = self . inner . read_wait . take () { waker . wake () ; } } _ => { } } } }
};
}
