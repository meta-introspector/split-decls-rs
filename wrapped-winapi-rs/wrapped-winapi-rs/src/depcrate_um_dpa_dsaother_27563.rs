// Generated macro for other_27563 (other)
macro_rules! Depcrate_um_dpa_dsaother_27563 {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"other_27563"}
// Dependencies: {}
extern "system" { pub fn DPA_Create (cItemGrow : c_int ,) -> HDPA ; pub fn DPA_CreateEx (cpGrow : c_int , hheap : HANDLE ,) -> HDPA ; pub fn DPA_Clone (hdpa : HDPA , hdpaNew : HDPA ,) -> HDPA ; pub fn DPA_Destroy (hdpa : HDPA ,) -> BOOL ; pub fn DPA_DestroyCallback (hdpa : HDPA , pfnCB : PFNDAENUMCALLBACK , pData : * mut c_void ,) ; pub fn DPA_DeletePtr (hdpa : HDPA , i : c_int ,) -> PVOID ; pub fn DPA_DeleteAllPtrs (hdpa : HDPA ,) -> BOOL ; pub fn DPA_EnumCallback (hdpa : HDPA , pfnCB : PFNDAENUMCALLBACK , pData : * mut c_void ,) ; pub fn DPA_Grow (hdpa : HDPA , cp : c_int ,) -> BOOL ; pub fn DPA_InsertPtr (hdpa : HDPA , i : c_int , p : * mut c_void ,) -> c_int ; pub fn DPA_SetPtr (hdpa : HDPA , i : c_int , p : * mut c_void ,) -> BOOL ; pub fn DPA_GetPtr (hdpa : HDPA , i : INT_PTR ,) -> PVOID ; pub fn DPA_GetPtrIndex (hdpa : HDPA , p : * const c_void ,) -> c_int ; }
};
}
