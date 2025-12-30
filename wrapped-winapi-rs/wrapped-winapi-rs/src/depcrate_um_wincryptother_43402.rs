// Generated macro for other_43402 (other)
macro_rules! Depcrate_um_wincryptother_43402 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43402"}
// Dependencies: {}
extern "system" { pub fn CertVerifyCTLUsage (dwEncodingType : DWORD , dwSubjectType : DWORD , pvSubject : * mut c_void , pSubjectUsage : PCTL_USAGE , dwFlags : DWORD , pVerifyUsagePara : PCTL_VERIFY_USAGE_PARA , pVerifyUsageStatus : PCTL_VERIFY_USAGE_STATUS ,) -> BOOL ; }
};
}
