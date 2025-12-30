// Generated macro for IsReparseTagNameSurrogate (function)
macro_rules! Depcrate_um_winntIsReparseTagNameSurrogate {
() => {
// Module: crate::um::winnt
// Provides: {"IsReparseTagNameSurrogate"}
// Dependencies: {}
# [inline] pub fn IsReparseTagNameSurrogate (_tag : DWORD) -> bool { (_tag & 0x20000000) != 0 }
};
}
