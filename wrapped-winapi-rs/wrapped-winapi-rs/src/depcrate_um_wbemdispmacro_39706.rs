// Generated macro for macro_39706 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39706 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39706"}
// Dependencies: {}
RIDL ! { # [uuid (0xd2f68443 , 0x85dc , 0x427e , 0x91 , 0xd8 , 0x36 , 0x65 , 0x54 , 0xcc , 0x75 , 0x4c)] interface ISWbemServicesEx (ISWbemServicesExVtbl) : ISWbemServices (ISWbemServicesVtbl) { fn Put (objWbemObject : * mut ISWbemObjectEx , iFlags : c_long , objWbemNamedValueSet : * mut IDispatch , objWbemObjectPath : * mut * mut ISWbemObjectPath ,) -> HRESULT , fn PutAsync (objWbemSink : * mut ISWbemSink , objWbemObject : * mut ISWbemObjectEx , iFlags : c_long , objWbemNamedValueSet : * mut IDispatch , objWbemAsyncContext : * mut IDispatch ,) -> HRESULT , } }
};
}
