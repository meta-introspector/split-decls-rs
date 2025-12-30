// Generated macro for macro_38987 (macro)
macro_rules! Depcrate_um_taskschdmacro_38987 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38987"}
// Dependencies: {}
RIDL ! { # [uuid (0xd45b0167 , 0x9653 , 0x4eef , 0xb9 , 0x4f , 0x07 , 0x32 , 0xca , 0x7a , 0xf2 , 0x51)] interface IEventTrigger (IEventTriggerVtbl) : ITrigger (ITriggerVtbl) { fn get_Subscription (pQuery : * mut BSTR ,) -> HRESULT , fn put_Subscription (pQuery : BSTR ,) -> HRESULT , fn get_Delay (pDelay : * mut BSTR ,) -> HRESULT , fn put_Delay (pDelay : BSTR ,) -> HRESULT , fn get_ValueQueries (ppNamedXPaths : * mut * mut ITaskNamedValueCollection ,) -> HRESULT , fn put_ValueQueries (ppNamedXPaths : * const ITaskNamedValueCollection ,) -> HRESULT , } }
};
}
