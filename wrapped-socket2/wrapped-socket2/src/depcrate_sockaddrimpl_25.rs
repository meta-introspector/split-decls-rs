// Generated macro for impl_25 (impl)
macro_rules! Depcrate_sockaddrimpl_25 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_25"}
// Dependencies: {}
impl std :: fmt :: Debug for SockAddrStorage { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("sockaddr_storage") . field ("ss_family" , & self . storage . ss_family) . finish_non_exhaustive () } }
};
}
