// Generated macro for other_10259 (other)
macro_rules! Depcrate_shared_sspiother_10259 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10259"}
// Dependencies: {}
extern "system" { pub fn ChangeAccountPasswordA (pszPackageName : * mut SEC_CHAR , pszDomainName : * mut SEC_CHAR , pszAccountName : * mut SEC_CHAR , pszOldPassword : * mut SEC_CHAR , pszNewPassword : * mut SEC_CHAR , bImpersonating : BOOLEAN , dwReserved : c_ulong , pOutput : PSecBufferDesc ,) -> SECURITY_STATUS ; }
};
}
