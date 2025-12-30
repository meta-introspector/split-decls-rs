// Generated macro for arch (module)
macro_rules! Depcrate_os_fuchsia_rawarch {
() => {
// Module: crate::os::fuchsia::raw
// Provides: {"arch"}
// Dependencies: {}
# [cfg (target_arch = "riscv64")] mod arch { # [stable (feature = "raw_ext" , since = "1.1.0")] pub use libc :: { blkcnt_t , blksize_t , ino_t , nlink_t , off_t , stat , time_t } ; }
};
}
