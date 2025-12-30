// Generated macro for test (module)
macro_rules! Depcrate_policytest {
() => {
// Module: crate::policy
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: policy :: SecPolicy ; use crate :: secure_transport :: SslProtocolSide ; # [test] fn create_ssl () { SecPolicy :: create_ssl (SslProtocolSide :: SERVER , Some ("certifi.org")) ; } }
};
}
