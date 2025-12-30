// Generated macro for ImageList_AddIcon (function)
macro_rules! Depcrate_um_commctrlImageList_AddIcon {
() => {
// Module: crate::um::commctrl
// Provides: {"ImageList_AddIcon"}
// Dependencies: {}
# [inline] pub unsafe fn ImageList_AddIcon (himl : HIMAGELIST , hicon : HICON) -> c_int { ImageList_ReplaceIcon (himl , - 1 , hicon) }
};
}
