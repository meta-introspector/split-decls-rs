// Generated macro for IsEqualGUID (function)
macro_rules! Depcrate_shared_guiddefIsEqualGUID {
() => {
// Module: crate::shared::guiddef
// Provides: {"IsEqualGUID"}
// Dependencies: {}
# [inline] pub fn IsEqualGUID (g1 : & GUID , g2 : & GUID) -> bool { let a = unsafe { & * (g1 as * const _ as * const [u32 ; 4]) } ; let b = unsafe { & * (g2 as * const _ as * const [u32 ; 4]) } ; a [0] == b [0] && a [1] == b [1] && a [2] == b [2] && a [3] == b [3] }
};
}
