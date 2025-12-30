// Generated macro for macro_38973 (macro)
macro_rules! Depcrate_um_taskschdmacro_38973 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38973"}
// Dependencies: {}
RIDL ! { # [uuid (0x02820e19 , 0x7b98 , 0x4ed2 , 0xb2 , 0xe8 , 0xfd , 0xcc , 0xce , 0xff , 0x61 , 0x9b)] interface IActionCollection (IActionCollectionVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (pCount : * mut c_long ,) -> HRESULT , fn get_Item (index : c_long , ppAction : * mut * mut IAction ,) -> HRESULT , fn get__NewEnum (ppEnum : * mut LPUNKNOWN ,) -> HRESULT , fn get_XmlText (pText : * mut BSTR ,) -> HRESULT , fn put_XmlText (pText : BSTR ,) -> HRESULT , fn Create (Type : TASK_ACTION_TYPE , ppAction : * mut * mut IAction ,) -> HRESULT , fn Remove (index : VARIANT ,) -> HRESULT , fn Clear () -> HRESULT , fn get_Context (pContext : * mut BSTR ,) -> HRESULT , fn put_Context (pContext : BSTR ,) -> HRESULT , } }
};
}
