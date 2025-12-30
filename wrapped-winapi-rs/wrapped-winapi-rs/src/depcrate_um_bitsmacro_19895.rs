// Generated macro for macro_19895 (macro)
macro_rules! Depcrate_um_bitsmacro_19895 {
() => {
// Module: crate::um::bits
// Provides: {"macro_19895"}
// Dependencies: {}
RIDL ! { # [uuid (0x5ce34c0d , 0x0dc9 , 0x4c1f , 0x89 , 0x7c , 0xda , 0xa1 , 0xb7 , 0x8c , 0xee , 0x7c)] interface IBackgroundCopyManager (IBackgroundCopyManagerVtbl) : IUnknown (IUnknownVtbl) { fn CreateJob (DisplayName : LPCWSTR , Type : BG_JOB_TYPE , pJobId : * mut GUID , ppJob : * mut * mut IBackgroundCopyJob ,) -> HRESULT , fn GetJob (jobID : REFGUID , ppJob : * mut * mut IBackgroundCopyJob ,) -> HRESULT , fn EnumJobs (dwFlags : DWORD , ppEnum : * mut * mut IEnumBackgroundCopyJobs ,) -> HRESULT , fn GetErrorDescription (hResult : HRESULT , LanguageId : DWORD , pErrorDescription : * mut LPWSTR ,) -> HRESULT , } }
};
}
