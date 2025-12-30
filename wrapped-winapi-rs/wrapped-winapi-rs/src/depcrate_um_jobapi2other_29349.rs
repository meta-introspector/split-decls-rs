// Generated macro for other_29349 (other)
macro_rules! Depcrate_um_jobapi2other_29349 {
() => {
// Module: crate::um::jobapi2
// Provides: {"other_29349"}
// Dependencies: {}
extern "system" { pub fn CreateJobObjectW (lpJobAttributes : LPSECURITY_ATTRIBUTES , lpName : LPCWSTR ,) -> HANDLE ; pub fn FreeMemoryJobObject (Buffer : * mut VOID ,) -> () ; pub fn OpenJobObjectW (dwDesiredAccess : DWORD , bInheritHandle : BOOL , lpName : LPCWSTR ,) -> HANDLE ; pub fn AssignProcessToJobObject (hJob : HANDLE , hProcess : HANDLE ,) -> BOOL ; pub fn TerminateJobObject (hJob : HANDLE , uExitCode : UINT ,) -> BOOL ; pub fn SetInformationJobObject (hJob : HANDLE , JobObjectInformationClass : JOBOBJECTINFOCLASS , lpJobObjectInformation : LPVOID , cbJovObjectInformationLength : DWORD ,) -> BOOL ; pub fn SetIoRateControlInformationJobObject (hJob : HANDLE , IoRateControlInfo : * mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION ,) -> DWORD ; pub fn QueryInformationJobObject (hJob : HANDLE , JobObjectInformationClass : JOBOBJECTINFOCLASS , lpJobObjectInformation : LPVOID , cbJovObjectInformationLength : DWORD , lpReturnLength : LPDWORD ,) -> BOOL ; pub fn QueryIoRateControlInformationJobObject (hJob : HANDLE , VolumeName : PCWSTR , InfoBlocks : * mut * mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION , InfoBlockCount : * mut ULONG ,) -> DWORD ; }
};
}
