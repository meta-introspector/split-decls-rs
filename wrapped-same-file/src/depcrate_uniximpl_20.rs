// Generated macro for impl_20 (impl)
macro_rules! Depcrate_uniximpl_20 {
() => {
// Module: crate::unix
// Provides: {"impl_20"}
// Dependencies: {}
impl IntoRawFd for crate :: Handle { fn into_raw_fd (mut self) -> RawFd { self . 0 . file . take () . unwrap () . into_raw_fd () } }
};
}
