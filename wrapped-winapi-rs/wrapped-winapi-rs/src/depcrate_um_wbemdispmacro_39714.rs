// Generated macro for macro_39714 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39714 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39714"}
// Dependencies: {}
RIDL ! { # [uuid (0x5ad4bf92 , 0xdaab , 0x11d3 , 0xb3 , 0x8f , 0x00 , 0x10 , 0x5a , 0x1f , 0x47 , 0x3a)] interface ISWbemRefreshableItem (ISWbemRefreshableItemVtbl) : IDispatch (IDispatchVtbl) { fn get_Index (iIndex : * mut c_long ,) -> HRESULT , fn get_Refresher (objWbemRefresher : * mut * mut ISWbemRefresher ,) -> HRESULT , fn get_IsSet (bIsSet : * mut VARIANT_BOOL ,) -> HRESULT , fn get_Object (objWbemObject : * mut * mut ISWbemObjectEx ,) -> HRESULT , fn get_ObjectSet (objWbemObjectSet : * mut * mut ISWbemObjectSet ,) -> HRESULT , fn Remove (iFlags : c_long ,) -> HRESULT , } }
};
}
