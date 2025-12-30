// Generated macro for other_37708 (other)
macro_rules! Depcrate_um_setupapiother_37708 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37708"}
// Dependencies: {}
extern "system" { pub fn SetupInstallFileA (InfHandle : HINF , InfContext : PINFCONTEXT , SourceFile : PCSTR , SourcePathRoot : PCSTR , DestinationName : PCSTR , CopyStyle : DWORD , CopyMsgHandler : PSP_FILE_CALLBACK_A , Context : PVOID ,) -> BOOL ; pub fn SetupInstallFileW (InfHandle : HINF , InfContext : PINFCONTEXT , SourceFile : PCWSTR , SourcePathRoot : PCWSTR , DestinationName : PCWSTR , CopyStyle : DWORD , CopyMsgHandler : PSP_FILE_CALLBACK_W , Context : PVOID ,) -> BOOL ; pub fn SetupInstallFileExA (InfHandle : HINF , InfContext : PINFCONTEXT , SourceFile : PCSTR , SourcePathRoot : PCSTR , DestinationName : PCSTR , CopyStyle : DWORD , CopyMsgHandler : PSP_FILE_CALLBACK_A , Context : PVOID , FileWasInUse : PBOOL ,) -> BOOL ; pub fn SetupInstallFileExW (InfHandle : HINF , InfContext : PINFCONTEXT , SourceFile : PCWSTR , SourcePathRoot : PCWSTR , DestinationName : PCWSTR , CopyStyle : DWORD , CopyMsgHandler : PSP_FILE_CALLBACK_W , Context : PVOID , FileWasInUse : PBOOL ,) -> BOOL ; }
};
}
