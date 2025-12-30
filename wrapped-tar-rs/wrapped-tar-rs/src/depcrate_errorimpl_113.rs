// Generated macro for impl_113 (impl)
macro_rules! Depcrate_errorimpl_113 {
() => {
// Module: crate::error
// Provides: {"impl_113"}
// Dependencies: {}
impl TarError { pub fn new (desc : impl Into < Cow < 'static , str > > , err : Error) -> TarError { TarError { desc : desc . into () , io : err , } } }
};
}
