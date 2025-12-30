// Generated macro for macro_33939 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33939 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33939"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000002b , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IWaitMultiple (IWaitMultipleVtbl) : IUnknown (IUnknownVtbl) { fn WaitMultiple (timeout : DWORD , pSync : * mut * mut ISynchronize ,) -> HRESULT , fn AddSynchronize (pSync : * mut ISynchronize ,) -> HRESULT , } }
};
}
