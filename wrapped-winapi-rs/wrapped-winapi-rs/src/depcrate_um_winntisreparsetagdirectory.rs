// Generated macro for IsReparseTagDirectory (function)
macro_rules! Depcrate_um_winntIsReparseTagDirectory {
() => {
// Module: crate::um::winnt
// Provides: {"IsReparseTagDirectory"}
// Dependencies: {}
# [inline] pub fn IsReparseTagDirectory (_tag : DWORD) -> bool { (_tag & 0x10000000) != 0 }
};
}
