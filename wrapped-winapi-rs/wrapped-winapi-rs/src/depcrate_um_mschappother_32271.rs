// Generated macro for other_32271 (other)
macro_rules! Depcrate_um_mschappother_32271 {
() => {
// Module: crate::um::mschapp
// Provides: {"other_32271"}
// Dependencies: {}
extern "system" { pub fn MSChapSrvChangePassword (ServerName : PWSTR , UserName : PWSTR , LmOldPresent : BOOLEAN , LmOldOwfPassword : PLM_OWF_PASSWORD , LmNewOwfPassword : PLM_OWF_PASSWORD , NtOldOwfPassword : PNT_OWF_PASSWORD , NtNewOwfPassword : PNT_OWF_PASSWORD ,) -> DWORD ; pub fn MSChapSrvChangePassword2 (ServerName : PWSTR , UserName : PWSTR , NewPasswordEncryptedWithOldNt : PSAMPR_ENCRYPTED_USER_PASSWORD , OldNtOwfPasswordEncryptedWithNewNt : PENCRYPTED_NT_OWF_PASSWORD , LmPresent : BOOLEAN , NewPasswordEncryptedWithOldLm : PSAMPR_ENCRYPTED_USER_PASSWORD , OldLmOwfPasswordEncryptedWithNewLmOrNt : PENCRYPTED_LM_OWF_PASSWORD ,) -> DWORD ; }
};
}
