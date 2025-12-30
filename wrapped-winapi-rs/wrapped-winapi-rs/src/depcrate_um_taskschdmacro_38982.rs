// Generated macro for macro_38982 (macro)
macro_rules! Depcrate_um_taskschdmacro_38982 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38982"}
// Dependencies: {}
RIDL ! { # [uuid (0xb4ef826b , 0x63c3 , 0x46e4 , 0xa5 , 0x04 , 0xef , 0x69 , 0xe4 , 0xf7 , 0xea , 0x4d)] interface ITaskNamedValueCollection (ITaskNamedValueCollectionVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (pCount : * mut c_long ,) -> HRESULT , fn get_Item (index : LONG , ppPair : * mut * mut ITaskNamedValuePair ,) -> HRESULT , fn get__NewEnum (ppEnum : * mut LPUNKNOWN ,) -> HRESULT , fn Create (Name : BSTR , Value : BSTR , ppPair : * mut * mut ITaskNamedValuePair ,) -> HRESULT , fn Remove (index : LONG ,) -> HRESULT , fn Clear () -> HRESULT , } }
};
}
