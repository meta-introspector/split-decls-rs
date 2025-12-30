// Generated macro for macro_33882 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33882 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33882"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000101 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumString (IEnumStringVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut LPOLESTR , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumString ,) -> HRESULT , } }
};
}
