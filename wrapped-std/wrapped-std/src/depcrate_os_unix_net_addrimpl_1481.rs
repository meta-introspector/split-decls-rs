// Generated macro for impl_1481 (impl)
macro_rules! Depcrate_os_unix_net_addrimpl_1481 {
() => {
// Module: crate::os::unix::net::addr
// Provides: {"impl_1481"}
// Dependencies: {}
# [doc (cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin")))] # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [stable (feature = "unix_socket_abstract" , since = "1.70.0")] impl linux_ext :: addr :: SocketAddrExt for SocketAddr { fn as_abstract_name (& self) -> Option < & [u8] > { if let AddressKind :: Abstract (name) = self . address () { Some (name . as_bytes ()) } else { None } } fn from_abstract_name < N > (name : N) -> crate :: io :: Result < Self > where N : AsRef < [u8] > , { let name = name . as_ref () ; unsafe { let mut addr : libc :: sockaddr_un = mem :: zeroed () ; addr . sun_family = libc :: AF_UNIX as libc :: sa_family_t ; if name . len () + 1 > addr . sun_path . len () { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "abstract socket name must be shorter than SUN_LEN" ,)) ; } crate :: ptr :: copy_nonoverlapping (name . as_ptr () , addr . sun_path . as_mut_ptr () . add (1) as * mut u8 , name . len () ,) ; let len = (SUN_PATH_OFFSET + 1 + name . len ()) as libc :: socklen_t ; SocketAddr :: from_parts (addr , len) } } }
};
}
