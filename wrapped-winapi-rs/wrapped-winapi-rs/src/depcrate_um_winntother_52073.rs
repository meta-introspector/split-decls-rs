// Generated macro for other_52073 (other)
macro_rules! Depcrate_um_winntother_52073 {
() => {
// Module: crate::um::winnt
// Provides: {"other_52073"}
// Dependencies: {}
extern "system" { pub fn RtlGetDeviceFamilyInfoEnum (pullUAPInfo : * mut ULONGLONG , pulDeviceFamily : * mut DWORD , pulDeviceForm : * mut DWORD ,) ; pub fn RtlConvertDeviceFamilyInfoToString (pulDeviceFamilyBufferSize : PDWORD , pulDeviceFormBufferSize : PDWORD , DeviceFamily : PWSTR , DeviceForm : PWSTR ,) -> DWORD ; pub fn RtlSwitchedVVI (VersionInfo : PRTL_OSVERSIONINFOEXW , TypeMask : DWORD , ConditionMask : ULONGLONG ,) -> DWORD ; }
};
}
