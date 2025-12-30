// Generated macro for macro_33834 (macro)
macro_rules! Depcrate_um_objidlmacro_33834 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33834"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000000d , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumSTATSTG (IEnumSTATSTGVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut STATSTG , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumSTATSTG ,) -> HRESULT , } }
};
}
