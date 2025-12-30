// Generated macro for macro_38954 (macro)
macro_rules! Depcrate_um_taskschdmacro_38954 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38954"}
// Dependencies: {}
RIDL ! { # [uuid (0x8cfac062 , 0xa080 , 0x4c15 , 0x9a , 0x88 , 0xaa , 0x7c , 0x2a , 0xf8 , 0x0d , 0xfc)] interface ITaskFolder (ITaskFolderVtbl) : IDispatch (IDispatchVtbl) { fn get_Name (pName : * mut BSTR ,) -> HRESULT , fn get_Path (pPath : * mut BSTR ,) -> HRESULT , fn GetFolder (Path : BSTR , ppFolder : * mut * mut ITaskFolder ,) -> HRESULT , fn GetFolders (flags : LONG , ppFolders : * mut * mut ITaskFolderCollection ,) -> HRESULT , fn CreateFolder (subFolderName : BSTR , sddl : VARIANT , ppFolder : * mut * mut ITaskFolder ,) -> HRESULT , fn DeleteFolder (subFolderName : BSTR , flags : LONG ,) -> HRESULT , fn GetTask (Path : BSTR , ppTask : * mut * mut IRegisteredTask ,) -> HRESULT , fn GetTasks (flags : LONG , ppTasks : * mut * mut IRegisteredTaskCollection ,) -> HRESULT , fn DeleteTask (Name : BSTR , flags : LONG ,) -> HRESULT , fn RegisterTask (Path : BSTR , XmlText : BSTR , flags : LONG , UserId : VARIANT , password : VARIANT , LogonType : TASK_LOGON_TYPE , sddl : VARIANT , ppTask : * mut * mut IRegisteredTask ,) -> HRESULT , fn RegisterTaskDefinition (Path : BSTR , pDefinition : * const ITaskDefinition , flags : LONG , UserId : VARIANT , password : VARIANT , LogonType : TASK_LOGON_TYPE , sddl : VARIANT , ppTask : * mut * mut IRegisteredTask ,) -> HRESULT , fn GetSecurityDescriptor (securityInformation : LONG , pSddl : * mut BSTR ,) -> HRESULT , fn SetSecurityDescriptor (sddl : BSTR , flags : LONG ,) -> HRESULT , } }
};
}
