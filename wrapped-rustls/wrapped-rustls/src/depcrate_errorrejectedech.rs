// Generated macro for RejectedEch (struct)
macro_rules! Depcrate_errorRejectedEch {
() => {
// Module: crate::error
// Provides: {"RejectedEch"}
// Dependencies: {}
# [doc = " The server rejected the request to enable Encrypted Client Hello (ECH)"] # [doc = ""] # [doc = " If [`RejectedEch::can_retry()`] is true, then you may use this with"] # [doc = " [`crate::client::EchConfig::for_retry()`] to build a new `EchConfig` for a fresh client"] # [doc = " connection that will use a compatible ECH configuration provided by the server for a retry."] # [non_exhaustive] # [derive (Debug , Clone , PartialEq)] pub struct RejectedEch { pub (crate) retry_configs : Option < Vec < EchConfigPayload > > , }
};
}
