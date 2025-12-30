// Generated macro for impl_1584 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1584 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1584"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for UnixStream { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("UnixStream") ; builder . field ("fd" , self . 0 . as_inner ()) ; if let Ok (addr) = self . local_addr () { builder . field ("local" , & addr) ; } if let Ok (addr) = self . peer_addr () { builder . field ("peer" , & addr) ; } builder . finish () } }
};
}
