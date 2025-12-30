// Generated macro for other_34482 (other)
macro_rules! Depcrate_um_powrprofother_34482 {
() => {
// Module: crate::um::powrprof
// Provides: {"other_34482"}
// Dependencies: {}
extern "system" { pub fn DevicePowerEnumDevices (QueryIndex : ULONG , QueryInterpretationFlags : ULONG , QueryFlags : ULONG , pReturnBuffer : PBYTE , pBufferSize : PULONG ,) -> BOOLEAN ; pub fn DevicePowerSetDeviceState (DeviceDescription : LPCWSTR , SetFlags : ULONG , SetData : PVOID ,) -> DWORD ; pub fn DevicePowerOpen (DebugMask : ULONG ,) -> BOOLEAN ; pub fn DevicePowerClose () -> BOOLEAN ; }
};
}
