// Generated macro for other_21254 (other)
macro_rules! Depcrate_um_commctrlother_21254 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_21254"}
// Dependencies: {}
extern "system" { pub fn ImageList_Copy (himlDst : HIMAGELIST , iDst : c_int , himlSrc : HIMAGELIST , iSrc : c_int , uFlags : UINT ,) -> BOOL ; pub fn ImageList_BeginDrag (himlTrack : HIMAGELIST , iTrack : c_int , dxHotspot : c_int , dyHotspot : c_int ,) -> BOOL ; pub fn ImageList_EndDrag () ; pub fn ImageList_DragEnter (hwndLock : HWND , x : c_int , y : c_int ,) -> BOOL ; pub fn ImageList_DragLeave (hwndLock : HWND ,) -> BOOL ; pub fn ImageList_DragMove (x : c_int , y : c_int ,) -> BOOL ; pub fn ImageList_SetDragCursorImage (himlDrag : HIMAGELIST , iDrag : c_int , dxHotspot : c_int , dyHotspot : c_int ,) -> BOOL ; pub fn ImageList_DragShowNolock (fShow : BOOL ,) -> BOOL ; pub fn ImageList_GetDragImage (ppt : * mut POINT , pptHotspot : * mut POINT ,) -> HIMAGELIST ; }
};
}
