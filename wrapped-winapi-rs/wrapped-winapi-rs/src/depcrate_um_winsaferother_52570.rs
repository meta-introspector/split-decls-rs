// Generated macro for other_52570 (other)
macro_rules! Depcrate_um_winsaferother_52570 {
() => {
// Module: crate::um::winsafer
// Provides: {"other_52570"}
// Dependencies: {}
extern "system" { pub fn SaferGetPolicyInformation (dwScopeId : DWORD , SaferPolicyInfoClass : SAFER_POLICY_INFO_CLASS , InfoBufferSize : DWORD , InfoBuffer : PVOID , InfoBufferRetSize : PDWORD , lpReserved : LPVOID ,) -> BOOL ; pub fn SaferSetPolicyInformation (dwScopeId : DWORD , SaferPolicyInfoClass : SAFER_POLICY_INFO_CLASS , InfoBufferSize : DWORD , InfoBuffer : PVOID , lpReserved : LPVOID ,) -> BOOL ; pub fn SaferCreateLevel (dwScopeId : DWORD , dwLevelId : DWORD , OpenFlags : DWORD , pLevelHandle : * mut SAFER_LEVEL_HANDLE , lpReserved : LPVOID ,) -> BOOL ; pub fn SaferCloseLevel (hLevelHandle : SAFER_LEVEL_HANDLE ,) -> BOOL ; pub fn SaferIdentifyLevel (dwNumProperties : DWORD , pCodeProperties : PSAFER_CODE_PROPERTIES , pLevelHandle : * mut SAFER_LEVEL_HANDLE , lpReserved : LPVOID ,) -> BOOL ; pub fn SaferComputeTokenFromLevel (LevelHandle : SAFER_LEVEL_HANDLE , InAccessToken : HANDLE , OutAccessToken : PHANDLE , dwFlags : DWORD , lpReserved : LPVOID ,) -> BOOL ; pub fn SaferGetLevelInformation (LevelHandle : SAFER_LEVEL_HANDLE , dwInfoType : SAFER_OBJECT_INFO_CLASS , lpQueryBuffer : LPVOID , dwInBufferSize : DWORD , lpdwOutBufferSize : LPDWORD ,) -> BOOL ; pub fn SaferSetLevelInformation (LevelHandle : SAFER_LEVEL_HANDLE , dwInfoType : SAFER_OBJECT_INFO_CLASS , lpQueryBuffer : LPVOID , dwInBufferSize : DWORD ,) -> BOOL ; pub fn SaferRecordEventLogEntry (hLevel : SAFER_LEVEL_HANDLE , szTargetPath : LPCWSTR , lpReserved : LPVOID ,) -> BOOL ; pub fn SaferiIsExecutableFileType (szFullPath : LPCWSTR , bFromShellExecute : BOOLEAN ,) -> BOOL ; }
};
}
