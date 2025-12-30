// Generated macro for other_30094 (other)
macro_rules! Depcrate_um_lmatother_30094 {
() => {
// Module: crate::um::lmat
// Provides: {"other_30094"}
// Dependencies: {}
extern "system" { pub fn NetScheduleJobAdd (Servername : LPCWSTR , Buffer : LPBYTE , JobId : LPDWORD ,) -> NET_API_STATUS ; pub fn NetScheduleJobDel (Servername : LPCWSTR , MinJobId : DWORD , MaxJobId : DWORD ,) -> NET_API_STATUS ; pub fn NetScheduleJobEnum (Servername : LPCWSTR , PointerToBuffer : * mut LPBYTE , PointerToBuffer : DWORD , EntriesRead : LPDWORD , TotalEntries : LPDWORD , ResumeHandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetScheduleJobGetInfo (Servername : LPCWSTR , JobId : DWORD , PointerToBuffer : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
