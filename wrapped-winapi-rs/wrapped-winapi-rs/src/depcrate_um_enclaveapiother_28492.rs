// Generated macro for other_28492 (other)
macro_rules! Depcrate_um_enclaveapiother_28492 {
() => {
// Module: crate::um::enclaveapi
// Provides: {"other_28492"}
// Dependencies: {}
extern "system" { pub fn IsEnclaveTypeSupported (flEnclaveType : DWORD ,) -> BOOL ; pub fn CreateEnclave (hProcess : HANDLE , lpAddress : LPVOID , dwSize : SIZE_T , dwInitialCommitment : SIZE_T , flEnclaveType : DWORD , lpEnclaveInformation : LPCVOID , dwInfoLength : DWORD , lpEnclaveError : LPDWORD ,) -> LPVOID ; pub fn LoadEnclaveData (hProcess : HANDLE , lpAddress : LPVOID , lpBuffer : LPCVOID , nSize : SIZE_T , flProtect : DWORD , lpPageInformation : LPCVOID , dwInfoLength : DWORD , lpNumberOfBytesWritten : PSIZE_T , lpEnclaveError : LPDWORD ,) -> BOOL ; pub fn InitializeEnclave (hProcess : HANDLE , lpAddress : LPVOID , lpEnclaveInformation : LPCVOID , dwInfoLength : DWORD , lpEnclaveError : LPDWORD ,) -> BOOL ; pub fn LoadEnclaveImageA (lpEnclaveAddress : LPVOID , lpImageName : LPCSTR ,) -> BOOL ; pub fn LoadEnclaveImageW (lpEnclaveAddress : LPVOID , lpImageName : LPCWSTR ,) -> BOOL ; pub fn CallEnclave (lpRoutine : LPENCLAVE_ROUTINE , lpParameter : LPVOID , fWaitForThread : BOOL , lpReturnValue : * mut LPVOID ,) -> BOOL ; pub fn TerminateEnclave (lpAddress : LPVOID , fWait : BOOL ,) -> BOOL ; pub fn DeleteEnclave (lpAddress : LPVOID ,) -> BOOL ; }
};
}
