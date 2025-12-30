// Generated macro for GDI_WIDTHBYTES (function)
macro_rules! Depcrate_um_wingdiGDI_WIDTHBYTES {
() => {
// Module: crate::um::wingdi
// Provides: {"GDI_WIDTHBYTES"}
// Dependencies: {}
# [inline] pub fn GDI_WIDTHBYTES (bits : DWORD) -> DWORD { ((bits + 31) & ! 31) / 8 }
};
}
