// Generated macro for UTIME_NOW (const)
macro_rules! Depcrate_fs_atUTIME_NOW {
() => {
// Module: crate::fs::at
// Provides: {"UTIME_NOW"}
// Dependencies: {}
# [doc = " `UTIME_NOW` for use with [`utimensat`]."] # [doc = ""] # [doc = " [`utimensat`]: crate::fs::utimensat"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "redox" , target_os = "vita")))] pub const UTIME_NOW : Nsecs = backend :: c :: UTIME_NOW as Nsecs ;
};
}
