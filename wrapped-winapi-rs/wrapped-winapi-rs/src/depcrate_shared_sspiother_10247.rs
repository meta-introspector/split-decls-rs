// Generated macro for other_10247 (other)
macro_rules! Depcrate_shared_sspiother_10247 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10247"}
// Dependencies: {}
extern "system" { pub fn AcquireCredentialsHandleW (pszPrincipal : LPWSTR , pszPackage : LPWSTR , fCredentialUse : c_ulong , pvLogonId : * mut c_void , pAuthData : * mut c_void , pGetKeyFn : SEC_GET_KEY_FN , pvGetKeyArgument : * mut c_void , phCredential : PCredHandle , ptsExpiry : PTimeStamp ,) -> SECURITY_STATUS ; }
};
}
