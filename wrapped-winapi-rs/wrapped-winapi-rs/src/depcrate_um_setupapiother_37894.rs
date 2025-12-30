// Generated macro for other_37894 (other)
macro_rules! Depcrate_um_setupapiother_37894 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37894"}
// Dependencies: {}
extern "system" { pub fn SetupDiGetClassDevsA (ClassGuid : * const GUID , Enumerator : PCSTR , hwndParent : HWND , Flags : DWORD ,) -> HDEVINFO ; pub fn SetupDiGetClassDevsW (ClassGuid : * const GUID , Enumerator : PCWSTR , hwndParent : HWND , Flags : DWORD ,) -> HDEVINFO ; pub fn SetupDiGetClassDevsExA (ClassGuid : * const GUID , Enumerator : PCSTR , hwndParent : HWND , Flags : DWORD , DeviceInfoSet : HDEVINFO , MachineName : PCSTR , Reserved : PVOID ,) -> HDEVINFO ; pub fn SetupDiGetClassDevsExW (ClassGuid : * const GUID , Enumerator : PCWSTR , hwndParent : HWND , Flags : DWORD , DeviceInfoSet : HDEVINFO , MachineName : PCWSTR , Reserved : PVOID ,) -> HDEVINFO ; pub fn SetupDiGetINFClassA (InfName : PCSTR , ClassGuid : LPGUID , ClassName : PSTR , ClassNameSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupDiGetINFClassW (InfName : PCWSTR , ClassGuid : LPGUID , ClassName : PWSTR , ClassNameSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; }
};
}
