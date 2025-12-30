// Generated macro for macro_38968 (macro)
macro_rules! Depcrate_um_taskschdmacro_38968 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38968"}
// Dependencies: {}
RIDL ! { # [uuid (0x84594461 , 0x0053 , 0x4342 , 0xa8 , 0xfd , 0x08 , 0x8f , 0xab , 0xf1 , 0x1f , 0x32)] interface IIdleSettings (IIdleSettingsVtbl) : IDispatch (IDispatchVtbl) { fn get_IdleDuration (pDelay : * mut BSTR ,) -> HRESULT , fn put_IdleDuration (pDelay : BSTR ,) -> HRESULT , fn get_WaitTimeout (pTimeout : * mut BSTR ,) -> HRESULT , fn put_WaitTimeout (pTimeout : BSTR ,) -> HRESULT , fn get_StopOnIdleEnd (pStop : * mut VARIANT_BOOL ,) -> HRESULT , fn put_StopOnIdleEnd (pStop : VARIANT_BOOL ,) -> HRESULT , fn get_RestartOnIdle (pRestart : * mut VARIANT_BOOL ,) -> HRESULT , fn put_RestartOnIdle (pRestart : VARIANT_BOOL ,) -> HRESULT , } }
};
}
