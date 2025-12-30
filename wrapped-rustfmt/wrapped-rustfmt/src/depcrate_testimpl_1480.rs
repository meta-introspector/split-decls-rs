// Generated macro for impl_1480 (impl)
macro_rules! Depcrate_testimpl_1480 {
() => {
// Module: crate::test
// Provides: {"impl_1480"}
// Dependencies: {}
impl Drop for TempFile { fn drop (& mut self) { use std :: fs :: remove_file ; remove_file (& self . path) . expect ("couldn't delete temp file") ; } }
};
}
