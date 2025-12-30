// Generated macro for macro_19885 (macro)
macro_rules! Depcrate_um_bitsmacro_19885 {
() => {
// Module: crate::um::bits
// Provides: {"macro_19885"}
// Dependencies: {}
RIDL ! { # [uuid (0x1af4f612 , 0x3b71 , 0x466f , 0x8f , 0x58 , 0x7b , 0x6f , 0x73 , 0xac , 0x57 , 0xad)] interface IEnumBackgroundCopyJobs (IEnumBackgroundCopyJobsVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut IBackgroundCopyJob , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumBackgroundCopyJobs ,) -> HRESULT , fn GetCount (puCount : * mut ULONG ,) -> HRESULT , } }
};
}
