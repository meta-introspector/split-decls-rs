// Generated macro for macro_19959 (macro)
macro_rules! Depcrate_um_bits3_0macro_19959 {
() => {
// Module: crate::um::bits3_0
// Provides: {"macro_19959"}
// Dependencies: {}
RIDL ! { # [uuid (0x659cdeaf , 0x489e , 0x11d9 , 0xa9 , 0xcd , 0x00 , 0x0d , 0x56 , 0x96 , 0x52 , 0x51)] interface IBitsPeerCacheRecord (IBitsPeerCacheRecordVtbl) : IUnknown (IUnknownVtbl) { fn GetId (pVal : * mut GUID ,) -> HRESULT , fn GetOriginUrl (pVal : * mut LPWSTR ,) -> HRESULT , fn GetFileSize (pVal : * mut UINT64 ,) -> HRESULT , fn GetFileModificationTime (pVal : * mut FILETIME ,) -> HRESULT , fn GetLastAccessTime (pVal : * mut FILETIME ,) -> HRESULT , fn IsFileValidated () -> HRESULT , fn GetFileRanges (pRangeCount : * mut DWORD , ppRanges : * mut * mut BG_FILE_RANGE ,) -> HRESULT , } }
};
}
