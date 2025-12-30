// Generated macro for ImageList_LoadBitmap (function)
macro_rules! Depcrate_um_commctrlImageList_LoadBitmap {
() => {
// Module: crate::um::commctrl
// Provides: {"ImageList_LoadBitmap"}
// Dependencies: {}
# [inline] pub unsafe fn ImageList_LoadBitmap (hi : HINSTANCE , lpbmp : LPCWSTR , cx : c_int , cGrow : c_int , crMask : COLORREF ,) -> HIMAGELIST { ImageList_LoadImageW (hi , lpbmp , cx , cGrow , crMask , IMAGE_BITMAP , 0) }
};
}
