// Generated macro for macro_33881 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33881 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33881"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000100 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumUnknown (IEnumUnknownVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut IUnknown , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumUnknown ,) -> HRESULT , } }
};
}
