// Generated macro for macro_34607 (macro)
macro_rules! Depcrate_um_propidlmacro_34607 {
() => {
// Module: crate::um::propidl
// Provides: {"macro_34607"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000139 , 0x0000 , 0x0000 , 0xC0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumSTATPROPSTG (IEnumSTATPROPSTGVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut STATPROPSTG , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Revert () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumSTATPROPSTG ,) -> HRESULT , } }
};
}
