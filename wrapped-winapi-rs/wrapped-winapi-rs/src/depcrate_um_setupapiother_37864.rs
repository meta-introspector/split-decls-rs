// Generated macro for other_37864 (other)
macro_rules! Depcrate_um_setupapiother_37864 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37864"}
// Dependencies: {}
extern "system" { pub fn SetupRemoveFileLogEntryA (FileLogHandle : HSPFILELOG , LogSectionName : PCSTR , TargetFilename : PCSTR ,) -> BOOL ; pub fn SetupRemoveFileLogEntryW (FileLogHandle : HSPFILELOG , LogSectionName : PCWSTR , TargetFilename : PCWSTR ,) -> BOOL ; }
};
}
