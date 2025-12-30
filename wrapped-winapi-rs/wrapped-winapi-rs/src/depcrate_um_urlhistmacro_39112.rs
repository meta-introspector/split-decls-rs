// Generated macro for macro_39112 (macro)
macro_rules! Depcrate_um_urlhistmacro_39112 {
() => {
// Module: crate::um::urlhist
// Provides: {"macro_39112"}
// Dependencies: {}
RIDL ! { # [uuid (0x3c374a42 , 0xbae4 , 0x11cf , 0xbf , 0x7d , 0x00 , 0xaa , 0x00 , 0x69 , 0x46 , 0xee)] interface IEnumSTATURL (IEnumSTATURLVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : LPSTATURL , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumSTATURL ,) -> HRESULT , fn SetFilter (poszFilter : LPCOLESTR , dwFlags : DWORD ,) -> HRESULT , } }
};
}
