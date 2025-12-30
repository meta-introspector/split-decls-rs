// Generated macro for impl_34 (impl)
macro_rules! Depcrate_sockaddrimpl_34 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_34"}
// Dependencies: {}
impl Hash for SockAddr { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_bytes () . hash (state) ; } }
};
}
