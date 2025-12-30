// Generated macro for macro_274 (macro)
macro_rules! Depcrate_flagmacro_274 {
() => {
// Module: crate::flag
// Provides: {"macro_274"}
// Dependencies: {}
bitflags :: bitflags ! { # [derive (Clone , Copy , Debug)] pub struct RecvFdFlags : usize { # [doc = " If set, the SYS_CALL payload specifies the destination file descriptor slots, otherwise the lowest"] # [doc = " available slots will be selected, and placed in the usize pointed to by SYS_CALL"] # [doc = " payload."] const MANUAL_FD = 1 ; # [doc = " If set, the file descriptors received will be placed into the *upper* file table."] const UPPER_TBL = 2 ; } }
};
}
