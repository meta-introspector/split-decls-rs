// Generated macro for other_37862 (other)
macro_rules! Depcrate_um_setupapiother_37862 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37862"}
// Dependencies: {}
extern "system" { pub fn SetupTerminateFileLog (FileLogHandle : HSPFILELOG ,) -> BOOL ; pub fn SetupLogFileA (FileLogHandle : HSPFILELOG , LogSectionName : PCSTR , SourceFilename : PCSTR , TargetFilename : PCSTR , Checksum : DWORD , DiskTagfile : PCSTR , DiskDescription : PCSTR , OtherInfo : PCSTR , Flags : DWORD ,) -> BOOL ; pub fn SetupLogFileW (FileLogHandle : HSPFILELOG , LogSectionName : PCWSTR , SourceFilename : PCWSTR , TargetFilename : PCWSTR , Checksum : DWORD , DiskTagfile : PCWSTR , DiskDescription : PCWSTR , OtherInfo : PCWSTR , Flags : DWORD ,) -> BOOL ; }
};
}
