// Generated macro for NetworkOracle (trait)
macro_rules! Depcrate_syscall_traitsNetworkOracle {
() => {
// Module: crate::syscall_traits
// Provides: {"NetworkOracle"}
// Dependencies: {}
# [doc = " Network operations oracle"] pub trait NetworkOracle { fn audit_connect () -> Result < () , String > ; fn check_endpoint_safety (endpoint : & str) -> bool ; }
};
}
