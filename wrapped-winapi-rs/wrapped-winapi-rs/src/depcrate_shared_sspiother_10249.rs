// Generated macro for other_10249 (other)
macro_rules! Depcrate_shared_sspiother_10249 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10249"}
// Dependencies: {}
extern "system" { pub fn AcquireCredentialsHandleA (pszPrincipal : LPSTR , pszPackage : LPSTR , fCredentialUse : c_ulong , pvLogonId : * mut c_void , pAuthData : * mut c_void , pGetKeyFn : SEC_GET_KEY_FN , pvGetKeyArgument : * mut c_void , phCredential : PCredHandle , ptsExpiry : PTimeStamp ,) -> SECURITY_STATUS ; }
};
}
