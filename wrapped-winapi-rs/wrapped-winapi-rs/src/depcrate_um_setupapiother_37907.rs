// Generated macro for other_37907 (other)
macro_rules! Depcrate_um_setupapiother_37907 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37907"}
// Dependencies: {}
extern "system" { pub fn SetupDiGetClassPropertyKeys (ClassGuid : * const GUID , PropertyKeyArray : * mut DEVPROPKEY , PropertyKeyCount : DWORD , RequiredPropertyKeyCount : PDWORD , Flags : DWORD ,) -> BOOL ; pub fn SetupDiGetClassPropertyKeysExW (ClassGuid : * const GUID , PropertyKeyArray : * mut DEVPROPKEY , PropertyKeyCount : DWORD , RequiredPropertyKeyCount : PDWORD , Flags : DWORD , MachineName : PCWSTR , Reserved : PVOID ,) -> BOOL ; pub fn SetupDiGetClassPropertyW (ClassGuid : * const GUID , PropertyKey : * const DEVPROPKEY , PropertyType : * mut DEVPROPTYPE , PropertyBuffer : PBYTE , PropertyBufferSize : DWORD , RequiredSize : PDWORD , Flags : DWORD ,) -> BOOL ; pub fn SetupDiGetClassPropertyExW (ClassGuid : * const GUID , PropertyKey : * const DEVPROPKEY , PropertyType : * mut DEVPROPTYPE , PropertyBuffer : PBYTE , PropertyBufferSize : DWORD , RequiredSize : PDWORD , Flags : DWORD , MachineName : PCWSTR , Reserved : PVOID ,) -> BOOL ; pub fn SetupDiSetClassPropertyW (ClassGuid : * const GUID , PropertyKey : * const DEVPROPKEY , PropertyType : DEVPROPTYPE , PropertyBuffer : * const BYTE , PropertyBufferSize : DWORD , Flags : DWORD ,) -> BOOL ; pub fn SetupDiSetClassPropertyExW (ClassGuid : * const GUID , PropertyKey : * const DEVPROPKEY , PropertyType : DEVPROPTYPE , PropertyBuffer : * const BYTE , PropertyBufferSize : DWORD , Flags : DWORD , MachineName : PCWSTR , Reserved : PVOID ,) -> BOOL ; }
};
}
