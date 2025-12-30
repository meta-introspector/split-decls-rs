// Generated macro for macro_33930 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33930 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33930"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000033 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface ISynchronizeContainer (ISynchronizeContainerVtbl) : IUnknown (IUnknownVtbl) { fn AddSynchronize (pSync : * mut ISynchronize ,) -> HRESULT , fn WaitMultiple (dwFlags : DWORD , dwTimeOut : DWORD , ppSync : * mut * mut ISynchronize ,) -> HRESULT , } }
};
}
