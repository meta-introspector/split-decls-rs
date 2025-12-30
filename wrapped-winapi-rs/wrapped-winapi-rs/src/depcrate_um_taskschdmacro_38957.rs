// Generated macro for macro_38957 (macro)
macro_rules! Depcrate_um_taskschdmacro_38957 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38957"}
// Dependencies: {}
RIDL ! { # [uuid (0x653758fb , 0x7b9a , 0x4f1e , 0xa4 , 0x71 , 0xbe , 0xeb , 0x8e , 0x9b , 0x83 , 0x4e)] interface IRunningTask (IRunningTaskVtbl) : IDispatch (IDispatchVtbl) { fn get_Name (pName : * mut BSTR ,) -> HRESULT , fn get_InstanceGuid (pGuid : * mut BSTR ,) -> HRESULT , fn get_Path (pPath : * mut BSTR ,) -> HRESULT , fn get_State (pState : * mut TASK_STATE ,) -> HRESULT , fn get_CurrentAction (pName : * mut BSTR ,) -> HRESULT , fn Stop () -> HRESULT , fn Refresh () -> HRESULT , fn get_EnginePID (pPID : * mut DWORD ,) -> HRESULT , } }
};
}
