// Generated macro for RtlCopyMemory (function)
macro_rules! Depcrate_um_winntRtlCopyMemory {
() => {
// Module: crate::um::winnt
// Provides: {"RtlCopyMemory"}
// Dependencies: {}
# [inline] pub unsafe fn RtlCopyMemory (Destination : * mut c_void , Source : * const c_void , Length : usize) { use core :: ptr :: copy_nonoverlapping ; copy_nonoverlapping (Source as * const u8 , Destination as * mut u8 , Length) ; }
};
}
