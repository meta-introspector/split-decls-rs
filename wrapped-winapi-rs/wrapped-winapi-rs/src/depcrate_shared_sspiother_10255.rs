// Generated macro for other_10255 (other)
macro_rules! Depcrate_shared_sspiother_10255 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10255"}
// Dependencies: {}
extern "system" { pub fn AddCredentialsA (hCredentials : PCredHandle , pszPrincipal : LPSTR , pszPackage : LPSTR , fCredentialUse : c_ulong , pAuthData : * mut c_void , pGetKeyFn : SEC_GET_KEY_FN , pvGetKeyArgument : * mut c_void , ptsExpiry : PTimeStamp ,) -> SECURITY_STATUS ; }
};
}
