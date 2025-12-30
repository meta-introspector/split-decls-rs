// Generated macro for macro_51575 (macro)
macro_rules! Depcrate_um_winntmacro_51575 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_51575"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] IFDEF ! { pub const IMAGE_ORDINAL_FLAG : ULONGLONG = IMAGE_ORDINAL_FLAG64 ; # [inline] pub fn IMAGE_ORDINAL (Ordinal : ULONGLONG) -> ULONGLONG { IMAGE_ORDINAL64 (Ordinal) } pub type IMAGE_THUNK_DATA = IMAGE_THUNK_DATA64 ; pub type PIMAGE_THUNK_DATA = PIMAGE_THUNK_DATA64 ; # [inline] pub fn IMAGE_SNAP_BY_ORDINAL (Ordinal : ULONGLONG) -> bool { IMAGE_SNAP_BY_ORDINAL64 (Ordinal) } pub type IMAGE_TLS_DIRECTORY = IMAGE_TLS_DIRECTORY64 ; pub type PIMAGE_TLS_DIRECTORY = PIMAGE_TLS_DIRECTORY64 ; }
};
}
