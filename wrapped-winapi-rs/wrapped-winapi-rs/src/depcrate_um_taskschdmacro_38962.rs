// Generated macro for macro_38962 (macro)
macro_rules! Depcrate_um_taskschdmacro_38962 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38962"}
// Dependencies: {}
RIDL ! { # [uuid (0x09941815 , 0xea89 , 0x4b5b , 0x89 , 0xe0 , 0x2a , 0x77 , 0x38 , 0x01 , 0xfa , 0xc3)] interface ITrigger (ITriggerVtbl) : IDispatch (IDispatchVtbl) { fn get_Type (pType : * mut TASK_TRIGGER_TYPE2 ,) -> HRESULT , fn get_Id (pId : * mut BSTR ,) -> HRESULT , fn put_Id (pId : BSTR ,) -> HRESULT , fn get_Repetition (ppRepeat : * mut * mut IRepetitionPattern ,) -> HRESULT , fn put_Repetition (ppRepeat : * const IRepetitionPattern ,) -> HRESULT , fn get_ExecutionTimeLimit (pTimeLimit : * mut BSTR ,) -> HRESULT , fn put_ExecutionTimeLimit (pTimeLimit : BSTR ,) -> HRESULT , fn get_StartBoundary (pStart : * mut BSTR ,) -> HRESULT , fn put_StartBoundary (pStart : BSTR ,) -> HRESULT , fn get_EndBoundary (pEnd : * mut BSTR ,) -> HRESULT , fn put_EndBoundary (pEnd : BSTR ,) -> HRESULT , fn get_Enabled (pEnabled : * mut VARIANT_BOOL ,) -> HRESULT , fn put_Enabled (pEnabled : VARIANT_BOOL ,) -> HRESULT , } }
};
}
