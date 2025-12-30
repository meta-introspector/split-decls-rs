// Generated macro for init_mib (function)
macro_rules! Depcrate_unix_freebsd_utilsinit_mib {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"init_mib"}
// Dependencies: {}
# [cfg (feature = "system")] # [inline] pub unsafe fn init_mib (name : & [u8] , mib : & mut [libc :: c_int]) { let mut len = mib . len () ; unsafe { libc :: sysctlnametomib (name . as_ptr () as _ , mib . as_mut_ptr () , & mut len) ; } }
};
}
