// Generated macro for other_21265 (other)
macro_rules! Depcrate_um_commctrlother_21265 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_21265"}
// Dependencies: {}
extern "system" { pub fn ImageList_GetIconSize (himl : HIMAGELIST , cx : * mut c_int , cy : * mut c_int ,) -> BOOL ; pub fn ImageList_SetIconSize (himl : HIMAGELIST , cx : c_int , cy : c_int ,) -> BOOL ; pub fn ImageList_GetImageInfo (himl : HIMAGELIST , i : c_int , pImageInfo : * mut IMAGEINFO ,) -> BOOL ; pub fn ImageList_Merge (himl1 : HIMAGELIST , i1 : c_int , himl2 : HIMAGELIST , i2 : c_int , dx : c_int , dy : c_int ,) -> HIMAGELIST ; pub fn ImageList_Duplicate (himl : HIMAGELIST ,) -> HIMAGELIST ; pub fn HIMAGELIST_QueryInterface (himl : HIMAGELIST , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT ; }
};
}
