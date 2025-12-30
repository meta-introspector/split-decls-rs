// Generated macro for other_10261 (other)
macro_rules! Depcrate_shared_sspiother_10261 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10261"}
// Dependencies: {}
extern "system" { pub fn InitializeSecurityContextW (phCredential : PCredHandle , phContext : PCtxtHandle , pszTargetName : * mut SEC_WCHAR , fContextReq : c_ulong , Reserved1 : c_ulong , TargetDataRep : c_ulong , pInput : PSecBufferDesc , Reserved2 : c_ulong , phNewContext : PCtxtHandle , pOutput : PSecBufferDesc , pfContextAttr : * mut c_ulong , ptsExpiry : PTimeStamp ,) -> SECURITY_STATUS ; }
};
}
