// Generated macro for ImageList_ExtractIcon (function)
macro_rules! Depcrate_um_commctrlImageList_ExtractIcon {
() => {
// Module: crate::um::commctrl
// Provides: {"ImageList_ExtractIcon"}
// Dependencies: {}
# [inline] pub unsafe fn ImageList_ExtractIcon (_ : HINSTANCE , himl : HIMAGELIST , i : c_int) -> HICON { ImageList_GetIcon (himl , i , 0) }
};
}
