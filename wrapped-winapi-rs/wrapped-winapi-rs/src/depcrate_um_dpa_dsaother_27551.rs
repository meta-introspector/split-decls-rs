// Generated macro for other_27551 (other)
macro_rules! Depcrate_um_dpa_dsaother_27551 {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"other_27551"}
// Dependencies: {}
extern "system" { pub fn DSA_Create (cbItem : c_int , cItemGrow : c_int ,) -> HDSA ; pub fn DSA_Destroy (hdsa : HDSA ,) -> BOOL ; pub fn DSA_DestroyCallback (hdsa : HDSA , pfnCB : PFNDAENUMCALLBACK , pData : * mut c_void ,) ; pub fn DSA_DeleteItem (hdsa : HDSA , i : c_int ,) -> BOOL ; pub fn DSA_DeleteAllItems (hdsa : HDSA ,) -> BOOL ; pub fn DSA_EnumCallback (hdsa : HDSA , pfnCB : PFNDAENUMCALLBACK , pData : * mut c_void ,) ; pub fn DSA_InsertItem (hdsa : HDSA , i : c_int , pitem : * const c_void ,) -> c_int ; pub fn DSA_GetItemPtr (hdsa : HDSA , i : c_int ,) -> PVOID ; pub fn DSA_GetItem (hdsa : HDSA , i : c_int , pitem : * mut c_void ,) -> BOOL ; pub fn DSA_SetItem (hdsa : HDSA , i : c_int , pitem : * const c_void ,) -> BOOL ; }
};
}
