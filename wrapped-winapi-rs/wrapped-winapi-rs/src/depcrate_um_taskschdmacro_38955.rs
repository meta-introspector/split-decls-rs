// Generated macro for macro_38955 (macro)
macro_rules! Depcrate_um_taskschdmacro_38955 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38955"}
// Dependencies: {}
RIDL ! { # [uuid (0x9c86f320 , 0xdee3 , 0x4dd1 , 0xb9 , 0x72 , 0xa3 , 0x03 , 0xf2 , 0x6b , 0x06 , 0x1e)] interface IRegisteredTask (IRegisteredTaskVtbl) : IDispatch (IDispatchVtbl) { fn get_Name (pName : * mut BSTR ,) -> HRESULT , fn get_Path (pPath : * mut BSTR ,) -> HRESULT , fn get_State (pState : * mut TASK_STATE ,) -> HRESULT , fn get_Enabled (pEnabled : * mut VARIANT_BOOL ,) -> HRESULT , fn put_Enabled (pEnabled : VARIANT_BOOL ,) -> HRESULT , fn Run (params : VARIANT , ppRunningTask : * mut * mut IRunningTask ,) -> HRESULT , fn RunEx (params : VARIANT , flags : LONG , sessionID : LONG , user : BSTR , ppRunningTask : * mut * mut IRunningTask ,) -> HRESULT , fn GetInstances (flags : LONG , ppRunningTasks : * mut * mut IRunningTaskCollection ,) -> HRESULT , fn get_LastRunTime (pLastRunTime : * mut DATE ,) -> HRESULT , fn get_LastTaskResult (pLastTaskResult : * mut LONG ,) -> HRESULT , fn get_NumberOfMissedRuns (pNumberOfMissedRuns : * mut LONG ,) -> HRESULT , fn get_NextRunTime (pNextRunTime : * mut DATE ,) -> HRESULT , fn get_Definition (ppDefinition : * mut * mut ITaskDefinition ,) -> HRESULT , fn get_Xml (pXml : * mut BSTR ,) -> HRESULT , fn GetSecurityDescriptor (securityInformation : LONG , pSddl : * mut BSTR ,) -> HRESULT , fn SetSecurityDescriptor (sddl : BSTR , flags : LONG ,) -> HRESULT , fn Stop (flags : LONG ,) -> HRESULT , fn GetRunTimes (pstStart : * const SYSTEMTIME , pstEnd : * const SYSTEMTIME , pCount : * mut DWORD , pRunTimes : * mut * mut SYSTEMTIME ,) -> HRESULT , } }
};
}
