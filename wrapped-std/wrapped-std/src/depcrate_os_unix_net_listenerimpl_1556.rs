// Generated macro for impl_1556 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1556 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1556"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for UnixListener { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("UnixListener") ; builder . field ("fd" , self . 0 . as_inner ()) ; if let Ok (addr) = self . local_addr () { builder . field ("local" , & addr) ; } builder . finish () } }
};
}
