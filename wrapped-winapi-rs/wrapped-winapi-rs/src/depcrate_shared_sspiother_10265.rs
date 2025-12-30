// Generated macro for other_10265 (other)
macro_rules! Depcrate_shared_sspiother_10265 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10265"}
// Dependencies: {}
extern "system" { pub fn ExportSecurityContext (phContext : PCtxtHandle , fFlags : ULONG , pPackedContext : PSecBuffer , pToken : * mut * mut c_void ,) -> SECURITY_STATUS ; pub fn ImportSecurityContextW (pszPackage : LPWSTR , pPackedContext : PSecBuffer , Token : * mut c_void , phContext : PCtxtHandle ,) -> SECURITY_STATUS ; pub fn ImportSecurityContextA (pszPackage : LPSTR , pPackedContext : PSecBuffer , Token : * mut c_void , phContext : PCtxtHandle ,) -> SECURITY_STATUS ; }
};
}
