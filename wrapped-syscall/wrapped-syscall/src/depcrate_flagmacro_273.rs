// Generated macro for macro_273 (macro)
macro_rules! Depcrate_flagmacro_273 {
() => {
// Module: crate::flag
// Provides: {"macro_273"}
// Dependencies: {}
bitflags :: bitflags ! { # [derive (Clone , Copy , Debug)] pub struct FobtainFdFlags : usize { # [doc = " If set, the SYS_CALL payload specifies the destination file descriptor slots, otherwise the lowest"] # [doc = " available slots will be selected, and placed in the usize pointed to by SYS_CALL"] # [doc = " payload."] const MANUAL_FD = 1 ; # [doc = " If set, the file descriptors received are guaranteed to be exclusively owned (by the file"] # [doc = " table the obtainer is running in)."] const EXCLUSIVE = 2 ; # [doc = " If set, the file descriptors received will be placed into the *upper* file table."] const UPPER_TBL = 4 ; } }
};
}
