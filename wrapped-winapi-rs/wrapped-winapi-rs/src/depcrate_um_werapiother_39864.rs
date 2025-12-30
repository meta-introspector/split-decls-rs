// Generated macro for other_39864 (other)
macro_rules! Depcrate_um_werapiother_39864 {
() => {
// Module: crate::um::werapi
// Provides: {"other_39864"}
// Dependencies: {}
extern "system" { pub fn WerRegisterFile (pwzFile : PCWSTR , regFileType : WER_REGISTER_FILE_TYPE , dwFlags : DWORD ,) -> HRESULT ; pub fn WerUnregisterFile (pwzFilePath : PCWSTR ,) -> HRESULT ; pub fn WerRegisterMemoryBlock (pvAddress : PVOID , dwSize : DWORD ,) -> HRESULT ; pub fn WerUnregisterMemoryBlock (pvAddress : PVOID ,) -> HRESULT ; pub fn WerSetFlags (dwFlags : DWORD ,) -> HRESULT ; pub fn WerGetFlags (hProcess : HANDLE , pdwFlags : PDWORD ,) -> HRESULT ; pub fn WerAddExcludedApplication (pwzExeName : PCWSTR , bAllUsers : BOOL ,) -> HRESULT ; pub fn WerRemoveExcludedApplication (pwzExeName : PCWSTR , bAllUsers : BOOL ,) -> HRESULT ; pub fn WerRegisterRuntimeExceptionModule (pwszOutOfProcessCallbackDll : PCWSTR , pContext : PVOID ,) -> HRESULT ; pub fn WerUnregisterRuntimeExceptionModule (pwszOutOfProcessCallbackDll : PCWSTR , pContext : PVOID ,) -> HRESULT ; }
};
}
