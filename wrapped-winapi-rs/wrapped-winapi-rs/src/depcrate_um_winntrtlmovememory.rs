// Generated macro for RtlMoveMemory (function)
macro_rules! Depcrate_um_winntRtlMoveMemory {
() => {
// Module: crate::um::winnt
// Provides: {"RtlMoveMemory"}
// Dependencies: {}
# [inline] pub unsafe fn RtlMoveMemory (Destination : * mut c_void , Source : * const c_void , Length : usize) { use core :: ptr :: copy ; copy (Source as * const u8 , Destination as * mut u8 , Length) ; }
};
}
