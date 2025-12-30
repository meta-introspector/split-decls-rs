// Generated macro for Nsecs (type)
macro_rules! Depcrate_timespecNsecs {
() => {
// Module: crate::timespec
// Provides: {"Nsecs"}
// Dependencies: {}
# [doc = " A type for the `tv_nsec` field of [`Timespec`]."] # [cfg (all (not (fix_y2038) , libc , not (all (target_arch = "x86_64" , target_pointer_width = "32"))))] pub type Nsecs = ffi :: c_long ;
};
}
