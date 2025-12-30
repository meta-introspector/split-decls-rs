// Generated macro for impl_1324 (impl)
macro_rules! Depcrate_netimpl_1324 {
() => {
// Module: crate::net
// Provides: {"impl_1324"}
// Dependencies: {}
impl Connection for std :: net :: TcpStream { # [inline] fn try_clone (& self) -> std :: io :: Result < Box < dyn Connection > > { let stream = std :: net :: TcpStream :: try_clone (self) ? ; Ok (Box :: new (stream)) } }
};
}
