// Generated macro for other_409 (other)
macro_rules! Depcrate_policyother_409 {
() => {
// Module: crate::policy
// Provides: {"other_409"}
// Dependencies: {}
extern "C" { pub fn SecPolicyCreateSSL (server : Boolean , hostname : CFStringRef) -> SecPolicyRef ; pub fn SecPolicyCreateRevocation (revocationFlags : CFOptionFlags) -> SecPolicyRef ; pub fn SecPolicyGetTypeID () -> CFTypeID ; pub fn SecPolicyCreateBasicX509 () -> SecPolicyRef ; }
};
}
