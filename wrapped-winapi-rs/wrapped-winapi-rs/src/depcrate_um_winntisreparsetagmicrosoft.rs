// Generated macro for IsReparseTagMicrosoft (function)
macro_rules! Depcrate_um_winntIsReparseTagMicrosoft {
() => {
// Module: crate::um::winnt
// Provides: {"IsReparseTagMicrosoft"}
// Dependencies: {}
# [inline] pub fn IsReparseTagMicrosoft (_tag : DWORD) -> bool { (_tag & 0x80000000) != 0 }
};
}
