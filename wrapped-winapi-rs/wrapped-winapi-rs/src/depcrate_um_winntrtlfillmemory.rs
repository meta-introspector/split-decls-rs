// Generated macro for RtlFillMemory (function)
macro_rules! Depcrate_um_winntRtlFillMemory {
() => {
// Module: crate::um::winnt
// Provides: {"RtlFillMemory"}
// Dependencies: {}
# [inline] pub unsafe fn RtlFillMemory (Destination : * mut c_void , Length : usize , Fill : u8) { use core :: ptr :: write_bytes ; write_bytes (Destination as * mut u8 , Fill , Length) ; }
};
}
