// Generated macro for macro_38978 (macro)
macro_rules! Depcrate_um_taskschdmacro_38978 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38978"}
// Dependencies: {}
RIDL ! { # [uuid (0x839d7762 , 0x5121 , 0x4009 , 0x92 , 0x34 , 0x4f , 0x0d , 0x19 , 0x39 , 0x4f , 0x04)] interface ITaskHandler (ITaskHandlerVtbl) : IUnknown (IUnknownVtbl) { fn Start (pHandlerServices : LPUNKNOWN , Data : BSTR ,) -> HRESULT , fn Stop (pRetCode : * mut HRESULT ,) -> HRESULT , fn Pause () -> HRESULT , fn Resume () -> HRESULT , } }
};
}
