// Generated macro for ULISet32 (function)
macro_rules! Depcrate_um_combaseapiULISet32 {
() => {
// Module: crate::um::combaseapi
// Provides: {"ULISet32"}
// Dependencies: {}
# [inline] pub fn ULISet32 (li : & mut ULARGE_INTEGER , v : DWORD) { unsafe { li . u_mut () . HighPart = 0 ; li . u_mut () . LowPart = v ; } }
};
}
