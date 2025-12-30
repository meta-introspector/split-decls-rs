// Generated macro for tests (module)
macro_rules! Depcrate_net_typestests {
() => {
// Module: crate::net::types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_sizes () { # [cfg (target_os = "linux")] use crate :: backend :: c ; use crate :: ffi :: c_int ; use crate :: net :: addr :: SocketAddrStorage ; use core :: mem :: transmute ; assert_eq_size ! (RawProtocol , c_int) ; assert_eq_size ! (Protocol , c_int) ; assert_eq_size ! (Option < RawProtocol >, c_int) ; assert_eq_size ! (Option < Protocol >, c_int) ; assert_eq_size ! (RawSocketType , c_int) ; assert_eq_size ! (SocketType , c_int) ; assert_eq_size ! (SocketFlags , c_int) ; assert_eq_size ! (SocketAddrStorage , c :: sockaddr_storage) ; # [allow (unsafe_code)] unsafe { let t : Option < Protocol > = None ; assert_eq ! (0_u32 , transmute ::< Option < Protocol >, u32 > (t)) ; let t : Option < Protocol > = Some (Protocol :: from_raw (RawProtocol :: new (4567) . unwrap ())) ; assert_eq ! (4567_u32 , transmute ::< Option < Protocol >, u32 > (t)) ; } # [cfg (linux_kernel)] assert_eq_size ! (UCred , libc :: ucred) ; # [cfg (target_os = "linux")] assert_eq_size ! (super :: xdp :: XdpUmemReg , c :: xdp_umem_reg) ; # [cfg (target_os = "linux")] assert_eq_size ! (super :: xdp :: XdpOptions , c :: xdp_options) ; # [cfg (target_os = "linux")] assert_eq_size ! (super :: xdp :: XdpDesc , c :: xdp_desc) ; } }
};
}
