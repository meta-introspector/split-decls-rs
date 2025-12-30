// Generated macro for macro_51576 (macro)
macro_rules! Depcrate_um_winntmacro_51576 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_51576"}
// Dependencies: {}
# [cfg (target_pointer_width = "32")] IFDEF ! { pub const IMAGE_ORDINAL_FLAG : DWORD = IMAGE_ORDINAL_FLAG32 ; # [inline] pub fn IMAGE_ORDINAL (Ordinal : DWORD) -> DWORD { IMAGE_ORDINAL32 (Ordinal) } pub type IMAGE_THUNK_DATA = IMAGE_THUNK_DATA32 ; pub type PIMAGE_THUNK_DATA = PIMAGE_THUNK_DATA32 ; # [inline] pub fn IMAGE_SNAP_BY_ORDINAL (Ordinal : DWORD) -> bool { IMAGE_SNAP_BY_ORDINAL32 (Ordinal) } pub type IMAGE_TLS_DIRECTORY = IMAGE_TLS_DIRECTORY32 ; pub type PIMAGE_TLS_DIRECTORY = PIMAGE_TLS_DIRECTORY32 ; }
};
}
