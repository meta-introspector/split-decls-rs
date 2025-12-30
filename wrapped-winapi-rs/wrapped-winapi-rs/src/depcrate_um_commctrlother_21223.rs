// Generated macro for other_21223 (other)
macro_rules! Depcrate_um_commctrlother_21223 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_21223"}
// Dependencies: {}
extern "system" { pub fn ImageList_Create (cx : c_int , cy : c_int , flags : UINT , cInitial : c_int , cGrow : c_int ,) -> HIMAGELIST ; pub fn ImageList_Destroy (himl : HIMAGELIST ,) -> BOOL ; pub fn ImageList_GetImageCount (himl : HIMAGELIST ,) -> c_int ; pub fn ImageList_SetImageCount (himl : HIMAGELIST , uNewCount : UINT ,) -> BOOL ; pub fn ImageList_Add (himl : HIMAGELIST , hbmImage : HBITMAP , hbmMask : HBITMAP ,) -> c_int ; pub fn ImageList_ReplaceIcon (himl : HIMAGELIST , i : c_int , hicon : HICON ,) -> c_int ; pub fn ImageList_SetBkColor (himl : HIMAGELIST , clrBk : COLORREF ,) -> COLORREF ; pub fn ImageList_GetBkColor (himl : HIMAGELIST ,) -> COLORREF ; pub fn ImageList_SetOverlayImage (himl : HIMAGELIST , iImage : c_int , iOverlay : c_int ,) -> BOOL ; }
};
}
