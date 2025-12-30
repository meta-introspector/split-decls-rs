// Generated macro for CERT_CHAIN_PARA (struct)
macro_rules! Depcrate_verification_windowsCERT_CHAIN_PARA {
() => {
// Module: crate::verification::windows
// Provides: {"CERT_CHAIN_PARA"}
// Dependencies: {}
# [allow (non_camel_case_types , non_snake_case)] # [repr (C)] struct CERT_CHAIN_PARA { pub cbSize : u32 , pub RequestedUsage : CERT_USAGE_MATCH , pub RequestedIssuancePolicy : CERT_USAGE_MATCH , pub dwUrlRetrievalTimeout : u32 , pub fCheckRevocationFreshnessTime : i32 , pub dwRevocationFreshnessTime : u32 , pub pftCacheResync : * mut FILETIME , # [cfg (not (target_vendor = "win7"))] pub pStrongSignPara : * const CERT_STRONG_SIGN_PARA , # [cfg (not (target_vendor = "win7"))] pub dwStrongSignFlags : u32 , }
};
}
