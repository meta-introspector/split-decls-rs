// Generated macro for other_37755 (other)
macro_rules! Depcrate_um_setupapiother_37755 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37755"}
// Dependencies: {}
extern "system" { pub fn SetupGetFileQueueCount (FileQueue : HSPFILEQ , SubQueueFileOp : UINT , NumOperations : PUINT ,) -> BOOL ; pub fn SetupGetFileQueueFlags (FileQueue : HSPFILEQ , Flags : PDWORD ,) -> BOOL ; pub fn SetupSetFileQueueFlags (FileQueue : HSPFILEQ , FlagMask : DWORD , Flags : DWORD ,) -> BOOL ; }
};
}
