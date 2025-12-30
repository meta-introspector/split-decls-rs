// Generated macro for other_37662 (other)
macro_rules! Depcrate_um_setupapiother_37662 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37662"}
// Dependencies: {}
extern "system" { pub fn SetupGetInfInformationA (InfSpec : LPCVOID , SearchControl : DWORD , ReturnBuffer : PSP_INF_INFORMATION , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupGetInfInformationW (InfSpec : LPCVOID , SearchControl : DWORD , ReturnBuffer : PSP_INF_INFORMATION , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; }
};
}
