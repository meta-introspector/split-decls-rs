// Generated macro for macro_38985 (macro)
macro_rules! Depcrate_um_taskschdmacro_38985 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38985"}
// Dependencies: {}
RIDL ! { # [uuid (0x754da71b , 0x4385 , 0x4475 , 0x9d , 0xd9 , 0x59 , 0x82 , 0x94 , 0xfa , 0x36 , 0x41)] interface ISessionStateChangeTrigger (ISessionStateChangeTriggerVtbl) : ITrigger (ITriggerVtbl) { fn get_Delay (pDelay : * mut BSTR ,) -> HRESULT , fn put_Delay (pDelay : BSTR ,) -> HRESULT , fn get_UserId (pUser : * mut BSTR ,) -> HRESULT , fn put_UserId (pUser : BSTR ,) -> HRESULT , fn get_StateChange (pType : * mut TASK_SESSION_STATE_CHANGE_TYPE ,) -> HRESULT , fn put_StateChange (pType : TASK_SESSION_STATE_CHANGE_TYPE ,) -> HRESULT , } }
};
}
