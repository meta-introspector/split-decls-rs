// Generated macro for macro_39000 (macro)
macro_rules! Depcrate_um_taskschdmacro_39000 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_39000"}
// Dependencies: {}
RIDL ! { # [uuid (0x248919ae , 0xe345 , 0x4a6d , 0x8a , 0xeb , 0xe0 , 0xd3 , 0x16 , 0x5c , 0x90 , 0x4e)] interface IPrincipal2 (IPrincipal2Vtbl) : IDispatch (IDispatchVtbl) { fn get_ProcessTokenSidType (pProcessTokenSidType : * mut TASK_PROCESSTOKENSID ,) -> HRESULT , fn put_ProcessTokenSidType (pProcessTokenSidType : TASK_PROCESSTOKENSID ,) -> HRESULT , fn get_RequiredPrivilegeCount (pCount : * mut c_long ,) -> HRESULT , fn get_RequiredPrivilege (index : c_long , pPrivilege : * mut BSTR ,) -> HRESULT , fn AddRequiredPrivilege (privilege : BSTR ,) -> HRESULT , } }
};
}
