// Generated macro for macro_33827 (macro)
macro_rules! Depcrate_um_objidlmacro_33827 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33827"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000102 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumMoniker (IEnumMonikerVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut IMoniker , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumMoniker ,) -> HRESULT , } }
};
}
