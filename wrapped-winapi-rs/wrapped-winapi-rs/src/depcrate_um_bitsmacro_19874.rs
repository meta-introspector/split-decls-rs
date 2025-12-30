// Generated macro for macro_19874 (macro)
macro_rules! Depcrate_um_bitsmacro_19874 {
() => {
// Module: crate::um::bits
// Provides: {"macro_19874"}
// Dependencies: {}
RIDL ! { # [uuid (0xca51e165 , 0xc365 , 0x424c , 0x8d , 0x41 , 0x24 , 0xaa , 0xa4 , 0xff , 0x3c , 0x40)] interface IEnumBackgroundCopyFiles (IEnumBackgroundCopyFilesVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut IBackgroundCopyFile , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumBackgroundCopyFiles ,) -> HRESULT , fn GetCount (puCount : * mut ULONG ,) -> HRESULT , } }
};
}
