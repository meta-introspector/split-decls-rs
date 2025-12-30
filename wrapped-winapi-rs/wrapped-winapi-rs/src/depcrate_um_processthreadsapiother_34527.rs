// Generated macro for other_34527 (other)
macro_rules! Depcrate_um_processthreadsapiother_34527 {
() => {
// Module: crate::um::processthreadsapi
// Provides: {"other_34527"}
// Dependencies: {}
extern "system" { pub fn GetThreadInformation (hThread : HANDLE , ThreadInformationClass : THREAD_INFORMATION_CLASS , ThreadInformation : LPVOID , ThreadInformationSize : DWORD ,) -> BOOL ; pub fn SetThreadInformation (hThread : HANDLE , ThreadInformationClass : THREAD_INFORMATION_CLASS , ThreadInformation : LPVOID , ThreadInformationSize : DWORD ,) -> BOOL ; pub fn IsProcessCritical (hProcess : HANDLE , Critical : PBOOL ,) -> BOOL ; pub fn SetProtectedPolicy (PolicyGuid : LPCGUID , PolicyValue : ULONG_PTR , OldPolicyValue : PULONG_PTR ,) -> BOOL ; pub fn QueryProtectedPolicy (PolicyGuid : LPCGUID , PolicyValue : PULONG_PTR ,) -> BOOL ; pub fn SetThreadIdealProcessor (hThread : HANDLE , dwIdealProcessor : DWORD ,) -> DWORD ; }
};
}
