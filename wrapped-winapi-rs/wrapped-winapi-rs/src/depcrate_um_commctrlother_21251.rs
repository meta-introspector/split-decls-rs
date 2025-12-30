// Generated macro for other_21251 (other)
macro_rules! Depcrate_um_commctrlother_21251 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_21251"}
// Dependencies: {}
extern "system" { pub fn ImageList_Replace (himl : HIMAGELIST , i : c_int , hbmImage : HBITMAP , hbmMask : HBITMAP ,) -> BOOL ; pub fn ImageList_AddMasked (himl : HIMAGELIST , hbmImage : HBITMAP , crMask : COLORREF ,) -> c_int ; pub fn ImageList_DrawEx (himl : HIMAGELIST , i : c_int , hdcDst : HDC , x : c_int , y : c_int , dx : c_int , dy : c_int , rgbBk : COLORREF , rgbFg : COLORREF , fStyle : UINT ,) -> BOOL ; pub fn ImageList_DrawIndirect (pimldp : * mut IMAGELISTDRAWPARAMS ,) -> BOOL ; pub fn ImageList_Remove (himl : HIMAGELIST , i : c_int ,) -> BOOL ; pub fn ImageList_GetIcon (himl : HIMAGELIST , i : c_int , flags : UINT ,) -> HICON ; pub fn ImageList_LoadImageA (hi : HINSTANCE , lpbmp : LPCSTR , cx : c_int , cGrow : c_int , crMask : COLORREF , uType : UINT , uFlags : UINT ,) -> HIMAGELIST ; pub fn ImageList_LoadImageW (hi : HINSTANCE , lpbmp : LPCWSTR , cx : c_int , cGrow : c_int , crMask : COLORREF , uType : UINT , uFlags : UINT ,) -> HIMAGELIST ; }
};
}
