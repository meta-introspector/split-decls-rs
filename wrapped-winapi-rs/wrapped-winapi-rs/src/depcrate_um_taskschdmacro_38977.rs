// Generated macro for macro_38977 (macro)
macro_rules! Depcrate_um_taskschdmacro_38977 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38977"}
// Dependencies: {}
RIDL ! { # [uuid (0x2faba4c7 , 0x4da9 , 0x4013 , 0x96 , 0x97 , 0x20 , 0xcc , 0x3f , 0xd4 , 0x0f , 0x85)] interface ITaskService (ITaskServiceVtbl) : IDispatch (IDispatchVtbl) { fn GetFolder (Path : BSTR , ppFolder : * mut * mut ITaskFolder ,) -> HRESULT , fn GetRunningTasks (flags : LONG , ppRunningTasks : * mut * mut IRunningTaskCollection ,) -> HRESULT , fn NewTask (flags : DWORD , ppDefinition : * mut * mut ITaskDefinition ,) -> HRESULT , fn Connect (serverName : VARIANT , user : VARIANT , domain : VARIANT , password : VARIANT ,) -> HRESULT , fn get_Connected (pConnected : * mut VARIANT_BOOL ,) -> HRESULT , fn get_TargetServer (pServer : * mut BSTR ,) -> HRESULT , fn get_ConnectedUser (pUser : * mut BSTR ,) -> HRESULT , fn get_ConnectedDomain (pDomain : * mut BSTR ,) -> HRESULT , fn get_HighestVersion (pVersion : * mut DWORD ,) -> HRESULT , } }
};
}
