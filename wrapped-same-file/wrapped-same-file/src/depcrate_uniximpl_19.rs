// Generated macro for impl_19 (impl)
macro_rules! Depcrate_uniximpl_19 {
() => {
// Module: crate::unix
// Provides: {"impl_19"}
// Dependencies: {}
impl AsRawFd for crate :: Handle { fn as_raw_fd (& self) -> RawFd { self . 0 . file . as_ref () . take () . unwrap () . as_raw_fd () } }
};
}
