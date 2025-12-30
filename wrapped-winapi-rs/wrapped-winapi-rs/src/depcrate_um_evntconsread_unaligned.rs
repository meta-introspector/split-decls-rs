// Generated macro for read_unaligned (function)
macro_rules! Depcrate_um_evntconsread_unaligned {
() => {
// Module: crate::um::evntcons
// Provides: {"read_unaligned"}
// Dependencies: {}
# [inline] unsafe fn read_unaligned < T > (src : * const T) -> T { use core :: { mem , ptr } ; let mut tmp : T = mem :: uninitialized () ; ptr :: copy_nonoverlapping (src as * const u8 , & mut tmp as * mut T as * mut u8 , mem :: size_of :: < T > () ,) ; tmp }
};
}
