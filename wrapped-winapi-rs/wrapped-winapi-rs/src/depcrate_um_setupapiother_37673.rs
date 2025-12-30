// Generated macro for other_37673 (other)
macro_rules! Depcrate_um_setupapiother_37673 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37673"}
// Dependencies: {}
extern "system" { pub fn SetupDecompressOrCopyFileA (SourceFileName : PCSTR , TargetFileName : PCSTR , CompressionType : PUINT ,) -> DWORD ; pub fn SetupDecompressOrCopyFileW (SourceFileName : PCWSTR , TargetFileName : PCWSTR , CompressionType : PUINT ,) -> DWORD ; pub fn SetupGetSourceFileLocationA (InfHandle : HINF , InfContext : PINFCONTEXT , FileName : PCSTR , SourceId : PUINT , ReturnBuffer : PSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupGetSourceFileLocationW (InfHandle : HINF , InfContext : PINFCONTEXT , FileName : PCWSTR , SourceId : PUINT , ReturnBuffer : PWSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupGetSourceFileSizeA (InfHandle : HINF , InfContext : PINFCONTEXT , FileName : PCSTR , Section : PCSTR , FileSize : PDWORD , RoundingFactor : UINT ,) -> BOOL ; pub fn SetupGetSourceFileSizeW (InfHandle : HINF , InfContext : PINFCONTEXT , FileName : PCWSTR , Section : PCWSTR , FileSize : PDWORD , RoundingFactor : UINT ,) -> BOOL ; pub fn SetupGetTargetPathA (InfHandle : HINF , InfContext : PINFCONTEXT , Section : PCSTR , ReturnBuffer : PSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; pub fn SetupGetTargetPathW (InfHandle : HINF , InfContext : PINFCONTEXT , Section : PCWSTR , ReturnBuffer : PWSTR , ReturnBufferSize : DWORD , RequiredSize : PDWORD ,) -> BOOL ; }
};
}
