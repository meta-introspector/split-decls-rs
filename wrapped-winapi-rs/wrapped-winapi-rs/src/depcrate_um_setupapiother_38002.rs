// Generated macro for other_38002 (other)
macro_rules! Depcrate_um_setupapiother_38002 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_38002"}
// Dependencies: {}
extern "system" { pub fn SetupVerifyInfFileA (InfName : PCSTR , AltPlatformInfo : PSP_ALTPLATFORM_INFO , InfSignerInfo : PSP_INF_SIGNER_INFO_A ,) -> BOOL ; pub fn SetupVerifyInfFileW (InfName : PCWSTR , AltPlatformInfo : PSP_ALTPLATFORM_INFO , InfSignerInfo : PSP_INF_SIGNER_INFO_W ,) -> BOOL ; }
};
}
