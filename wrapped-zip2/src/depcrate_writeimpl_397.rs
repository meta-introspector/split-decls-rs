// Generated macro for impl_397 (impl)
macro_rules! Depcrate_writeimpl_397 {
() => {
// Module: crate::write
// Provides: {"impl_397"}
// Dependencies: {}
impl < W : Write + Seek > Drop for ZipWriter < W > { fn drop (& mut self) { if ! self . inner . is_closed () { if let Err (e) = self . finalize () { let _ = write ! (io :: stderr () , "ZipWriter drop failed: {e:?}") ; } } } }
};
}
