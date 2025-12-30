// Generated macro for impl_1503 (impl)
macro_rules! Depcrate_testimpl_1503 {
() => {
// Module: crate::test
// Provides: {"impl_1503"}
// Dependencies: {}
impl Drop for TempFile { fn drop (& mut self) { use std :: fs :: remove_file ; remove_file (& self . path) . expect ("couldn't delete temp file") ; } }
};
}
