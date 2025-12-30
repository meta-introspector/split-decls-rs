// Generated macro for DPA_SortedInsertPtr (function)
macro_rules! Depcrate_um_dpa_dsaDPA_SortedInsertPtr {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"DPA_SortedInsertPtr"}
// Dependencies: {}
# [inline] pub unsafe fn DPA_SortedInsertPtr (hdpa : HDPA , pFind : * mut c_void , iStart : c_int , pfnCompare : PFNDACOMPARE , lParam : LPARAM , options : UINT , pitem : * mut c_void ,) -> c_int { DPA_InsertPtr (hdpa , DPA_Search (hdpa , pFind , iStart , pfnCompare , lParam , DPAS_SORTED | options ,) , pitem ,) }
};
}
