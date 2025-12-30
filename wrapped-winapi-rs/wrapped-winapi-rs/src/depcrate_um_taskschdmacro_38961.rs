// Generated macro for macro_38961 (macro)
macro_rules! Depcrate_um_taskschdmacro_38961 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38961"}
// Dependencies: {}
RIDL ! { # [uuid (0x85df5081 , 0x1b24 , 0x4f32 , 0x87 , 0x8a , 0xd9 , 0xd1 , 0x4d , 0xf4 , 0xcb , 0x77)] interface ITriggerCollection (ITriggerCollectionVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (pCount : * mut c_long ,) -> HRESULT , fn get_Item (index : c_long , ppTrigger : * mut * mut ITrigger ,) -> HRESULT , fn get__NewEnum (ppEnum : * mut LPUNKNOWN ,) -> HRESULT , fn Create (Type : TASK_TRIGGER_TYPE2 , ppTrigger : * mut * mut ITrigger ,) -> HRESULT , fn Remove (index : VARIANT ,) -> HRESULT , fn Clear () -> HRESULT , } }
};
}
