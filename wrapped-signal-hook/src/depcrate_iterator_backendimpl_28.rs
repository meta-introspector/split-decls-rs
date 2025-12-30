// Generated macro for impl_28 (impl)
macro_rules! Depcrate_iterator_backendimpl_28 {
() => {
// Module: crate::iterator::backend
// Provides: {"impl_28"}
// Dependencies: {}
impl < W : AsRawFd + Debug + Send + Sync > SelfPipeWrite for W { fn wake_readers (& self) { pipe :: wake (self . as_raw_fd () , WakeMethod :: Send) ; } }
};
}
