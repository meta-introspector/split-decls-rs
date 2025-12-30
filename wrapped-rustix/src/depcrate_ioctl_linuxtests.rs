// Generated macro for tests (module)
macro_rules! Depcrate_ioctl_linuxtests {
() => {
// Module: crate::ioctl::linux
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [allow (unused_imports)] use super :: * ; # [cfg (all (linux_raw_dep , not (any (target_arch = "sparc" , target_arch = "sparc64"))))] # [test] fn check_known_opcodes () { use crate :: backend :: c :: { c_long , c_uint } ; use core :: mem :: size_of ; assert_eq ! (compose_opcode (Direction :: Read , b'U' as Opcode , 15 , size_of ::< c_uint > () as Opcode) , linux_raw_sys :: ioctl :: USBDEVFS_CLAIMINTERFACE as Opcode) ; assert_eq ! (compose_opcode (Direction :: Write , b'v' as Opcode , 2 , size_of ::< c_long > () as Opcode) , linux_raw_sys :: ioctl :: FS_IOC_SETVERSION as Opcode) ; } }
};
}
