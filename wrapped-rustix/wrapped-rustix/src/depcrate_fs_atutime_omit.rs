// Generated macro for UTIME_OMIT (const)
macro_rules! Depcrate_fs_atUTIME_OMIT {
() => {
// Module: crate::fs::at
// Provides: {"UTIME_OMIT"}
// Dependencies: {}
# [doc = " `UTIME_OMIT` for use with [`utimensat`]."] # [doc = ""] # [doc = " [`utimensat`]: crate::fs::utimensat"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "redox" , target_os = "vita")))] pub const UTIME_OMIT : Nsecs = backend :: c :: UTIME_OMIT as Nsecs ;
};
}
