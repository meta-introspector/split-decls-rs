// Generated macro for other_10257 (other)
macro_rules! Depcrate_shared_sspiother_10257 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10257"}
// Dependencies: {}
extern "system" { pub fn ChangeAccountPasswordW (pszPackageName : * mut SEC_WCHAR , pszDomainName : * mut SEC_WCHAR , pszAccountName : * mut SEC_WCHAR , pszOldPassword : * mut SEC_WCHAR , pszNewPassword : * mut SEC_WCHAR , bImpersonating : BOOLEAN , dwReserved : c_ulong , pOutput : PSecBufferDesc ,) -> SECURITY_STATUS ; }
};
}
