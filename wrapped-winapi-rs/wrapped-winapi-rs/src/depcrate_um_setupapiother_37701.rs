// Generated macro for other_37701 (other)
macro_rules! Depcrate_um_setupapiother_37701 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37701"}
// Dependencies: {}
extern "system" { pub fn SetupGetSourceInfoA (InfHandle : HINF , SourceId : UINT , InfoDesired : UINT , ReturnBuffer : PSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupGetSourceInfoW (InfHandle : HINF , SourceId : UINT , InfoDesired : UINT , ReturnBuffer : PWSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; }
};
}
