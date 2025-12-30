// Generated macro for tests (module)
macro_rules! Depcrate_ioctltests {
() => {
// Module: crate::ioctl
// Provides: {"tests"}
// Dependencies: {}
# [cfg (linux_raw_dep)] # [cfg (not (any (target_arch = "sparc" , target_arch = "sparc64")))] # [cfg (test)] mod tests { use super :: * ; # [test] fn test_opcode_funcs () { assert_eq ! (linux_raw_sys :: ioctl :: TUNGETDEVNETNS as Opcode , opcode :: none (b'T' , 227)) ; assert_eq ! (linux_raw_sys :: ioctl :: FS_IOC_GETVERSION as Opcode , opcode :: read ::< c :: c_long > (b'v' , 1)) ; assert_eq ! (linux_raw_sys :: ioctl :: TUNSETNOCSUM as Opcode , opcode :: write ::< c :: c_int > (b'T' , 200)) ; assert_eq ! (linux_raw_sys :: ioctl :: FIFREEZE as Opcode , opcode :: read_write ::< c :: c_int > (b'X' , 119)) ; } }
};
}
