// Generated macro for LISet32 (function)
macro_rules! Depcrate_um_combaseapiLISet32 {
() => {
// Module: crate::um::combaseapi
// Provides: {"LISet32"}
// Dependencies: {}
# [inline] pub fn LISet32 (li : & mut LARGE_INTEGER , v : DWORD) { unsafe { li . u_mut () . HighPart = if (v as LONG) < 0 { - 1 } else { 0 } ; li . u_mut () . LowPart = v ; } }
};
}
