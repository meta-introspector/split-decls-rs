// Generated macro for impl_136 (impl)
macro_rules! Depcrate_sessionimpl_136 {
() => {
// Module: crate::session
// Provides: {"impl_136"}
// Dependencies: {}
# [cfg (unix)] impl AsRawFd for Session { fn as_raw_fd (& self) -> RawFd { let inner = self . inner () ; match inner . tcp . as_ref () { Some (tcp) => tcp . as_raw_fd () , None => panic ! ("tried to obtain raw fd without tcp stream set") , } } }
};
}
