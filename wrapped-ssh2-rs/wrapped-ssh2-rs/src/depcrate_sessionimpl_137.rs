// Generated macro for impl_137 (impl)
macro_rules! Depcrate_sessionimpl_137 {
() => {
// Module: crate::session
// Provides: {"impl_137"}
// Dependencies: {}
# [cfg (windows)] impl AsRawSocket for Session { fn as_raw_socket (& self) -> RawSocket { let inner = self . inner () ; match inner . tcp . as_ref () { Some (tcp) => tcp . as_raw_socket () , None => panic ! ("tried to obtain raw socket without tcp stream set") , } } }
};
}
