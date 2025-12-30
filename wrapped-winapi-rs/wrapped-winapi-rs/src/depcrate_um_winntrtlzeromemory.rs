// Generated macro for RtlZeroMemory (function)
macro_rules! Depcrate_um_winntRtlZeroMemory {
() => {
// Module: crate::um::winnt
// Provides: {"RtlZeroMemory"}
// Dependencies: {}
# [inline] pub unsafe fn RtlZeroMemory (Destination : * mut c_void , Length : usize) { use core :: ptr :: write_bytes ; write_bytes (Destination as * mut u8 , 0 , Length) ; }
};
}
