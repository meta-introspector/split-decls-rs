// Generated macro for other_10253 (other)
macro_rules! Depcrate_shared_sspiother_10253 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10253"}
// Dependencies: {}
extern "system" { pub fn AddCredentialsW (hCredentials : PCredHandle , pszPrincipal : LPWSTR , pszPackage : LPWSTR , fCredentialUse : c_ulong , pAuthData : * mut c_void , pGetKeyFn : SEC_GET_KEY_FN , pvGetKeyArgument : * mut c_void , ptsExpiry : PTimeStamp ,) -> SECURITY_STATUS ; }
};
}
