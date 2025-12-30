// Generated macro for macro_331 (macro)
macro_rules! Depcrate_flagmacro_331 {
() => {
// Module: crate::flag
// Provides: {"macro_331"}
// Dependencies: {}
bitflags ! { pub struct CallFlags : usize { const RSVD0 = 1 << 0 ; const RSVD1 = 1 << 1 ; const RSVD2 = 1 << 2 ; const RSVD3 = 1 << 3 ; const RSVD4 = 1 << 4 ; const RSVD5 = 1 << 5 ; const RSVD6 = 1 << 6 ; const RSVD7 = 1 << 7 ; # [doc = " Remove the fd from the caller's file table before sending the message."] const CONSUME = 1 << 8 ; const WRITE = 1 << 9 ; const READ = 1 << 10 ; # [doc = " Indicates the request is a bulk fd passing request."] const FD = 1 << 11 ; # [doc = " Flags for the fd passing request."] const FD_EXCLUSIVE = 1 << 12 ; const FD_CLONE = 1 << 13 ; const FD_UPPER = 1 << 14 ; } }
};
}
