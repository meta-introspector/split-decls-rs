// Generated macro for other_37768 (other)
macro_rules! Depcrate_um_setupapiother_37768 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37768"}
// Dependencies: {}
extern "system" { pub fn SetupUninstallOEMInfA (InfFileName : PCSTR , Flags : DWORD , Reserved : PVOID ,) -> BOOL ; pub fn SetupUninstallOEMInfW (InfFileName : PCWSTR , Flags : DWORD , Reserved : PVOID ,) -> BOOL ; pub fn SetupUninstallNewlyCopiedInfs (FileQueue : HSPFILEQ , Flags : DWORD , Reserved : PVOID ,) -> BOOL ; pub fn SetupCreateDiskSpaceListA (Reserved1 : PVOID , Reserved2 : DWORD , Flags : UINT ,) -> HDSKSPC ; pub fn SetupCreateDiskSpaceListW (Reserved1 : PVOID , Reserved2 : DWORD , Flags : UINT ,) -> HDSKSPC ; }
};
}
