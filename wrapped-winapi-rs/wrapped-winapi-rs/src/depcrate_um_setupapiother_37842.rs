// Generated macro for other_37842 (other)
macro_rules! Depcrate_um_setupapiother_37842 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37842"}
// Dependencies: {}
extern "system" { pub fn SetupInstallFilesFromInfSectionA (InfHandle : HINF , LayoutInfHandle : HINF , FileQueue : HSPFILEQ , SectionName : PCSTR , SourceRootPath : PCSTR , CopyFlags : UINT ,) -> BOOL ; pub fn SetupInstallFilesFromInfSectionW (InfHandle : HINF , LayoutInfHandle : HINF , FileQueue : HSPFILEQ , SectionName : PCWSTR , SourceRootPath : PCWSTR , CopyFlags : UINT ,) -> BOOL ; }
};
}
