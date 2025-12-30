// Generated macro for other_37866 (other)
macro_rules! Depcrate_um_setupapiother_37866 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37866"}
// Dependencies: {}
extern "system" { pub fn SetupQueryFileLogA (FileLogHandle : HSPFILELOG , LogSectionName : PCSTR , TargetFilename : PCSTR , DesiredInfo : SetupFileLogInfo , DataOut : PSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupQueryFileLogW (FileLogHandle : HSPFILELOG , LogSectionName : PCWSTR , TargetFilename : PCWSTR , DesiredInfo : SetupFileLogInfo , DataOut : PWSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; }
};
}
