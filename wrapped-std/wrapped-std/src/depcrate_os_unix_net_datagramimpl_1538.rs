// Generated macro for impl_1538 (impl)
macro_rules! Depcrate_os_unix_net_datagramimpl_1538 {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"impl_1538"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for UnixDatagram { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("UnixDatagram") ; builder . field ("fd" , self . 0 . as_inner ()) ; if let Ok (addr) = self . local_addr () { builder . field ("local" , & addr) ; } if let Ok (addr) = self . peer_addr () { builder . field ("peer" , & addr) ; } builder . finish () } }
};
}
