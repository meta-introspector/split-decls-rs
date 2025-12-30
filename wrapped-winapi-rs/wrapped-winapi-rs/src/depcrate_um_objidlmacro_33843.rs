// Generated macro for macro_33843 (macro)
macro_rules! Depcrate_um_objidlmacro_33843 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33843"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000105 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumSTATDATA (IEnumSTATDATAVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut STATDATA , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumSTATDATA ,) -> HRESULT , } }
};
}
