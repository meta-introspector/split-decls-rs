// Generated macro for other_19863 (other)
macro_rules! Depcrate_um_avrtother_19863 {
() => {
// Module: crate::um::avrt
// Provides: {"other_19863"}
// Dependencies: {}
extern "system" { pub fn AvSetMmThreadCharacteristicsA (TaskName : LPCSTR , TaskIndex : LPDWORD ,) -> HANDLE ; pub fn AvSetMmThreadCharacteristicsW (TaskName : LPCWSTR , TaskIndex : LPDWORD ,) -> HANDLE ; pub fn AvSetMmMaxThreadCharacteristicsA (FirstTask : LPCSTR , SecondTask : LPCSTR , TaskIndex : LPDWORD ,) -> HANDLE ; pub fn AvSetMmMaxThreadCharacteristicsW (FirstTask : LPCWSTR , SecondTask : LPCWSTR , TaskIndex : LPDWORD ,) -> HANDLE ; pub fn AvRevertMmThreadCharacteristics (avrt_handle : HANDLE ,) -> BOOL ; pub fn AvSetMmThreadPriority (AvrtHandle : HANDLE , Priority : AVRT_PRIORITY ,) -> BOOL ; pub fn AvRtCreateThreadOrderingGroup (Context : PHANDLE , Period : PLARGE_INTEGER , ThreadOrderingGuid : * mut GUID , Timeout : PLARGE_INTEGER ,) -> BOOL ; pub fn AvRtCreateThreadOrderingGroupExA (Context : PHANDLE , Period : PLARGE_INTEGER , ThreadOrderingGuid : * mut GUID , Timeout : PLARGE_INTEGER , TaskName : LPCSTR ,) -> BOOL ; pub fn AvRtCreateThreadOrderingGroupExW (Context : PHANDLE , Period : PLARGE_INTEGER , ThreadOrderingGuid : * mut GUID , Timeout : PLARGE_INTEGER , TaskName : LPCWSTR ,) -> BOOL ; pub fn AvRtJoinThreadOrderingGroup (Context : PHANDLE , ThreadOrderingGuid : * mut GUID , Before : BOOL ,) -> BOOL ; pub fn AvRtWaitOnThreadOrderingGroup (Context : HANDLE ,) -> BOOL ; pub fn AvRtLeaveThreadOrderingGroup (Context : HANDLE ,) -> BOOL ; pub fn AvRtDeleteThreadOrderingGroup (Context : HANDLE ,) -> BOOL ; pub fn AvQuerySystemResponsiveness (AvrtHandle : HANDLE , SystemResponsivenessValue : PULONG ,) -> BOOL ; }
};
}
