// Generated macro for other_38004 (other)
macro_rules! Depcrate_um_setupapiother_38004 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_38004"}
// Dependencies: {}
extern "system" { pub fn SetupDiGetCustomDevicePropertyA (DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , CustomPropertyName : PCSTR , Flags : DWORD , PropertyRegDataType : PDWORD , PropertyBuffer : PBYTE , PropertyBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupDiGetCustomDevicePropertyW (DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , CustomPropertyName : PCWSTR , Flags : DWORD , PropertyRegDataType : PDWORD , PropertyBuffer : PBYTE , PropertyBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; }
};
}
