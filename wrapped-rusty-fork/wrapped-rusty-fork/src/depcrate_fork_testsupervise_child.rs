// Generated macro for supervise_child (function)
macro_rules! Depcrate_fork_testsupervise_child {
() => {
// Module: crate::fork_test
// Provides: {"supervise_child"}
// Dependencies: {}
# [allow (missing_docs)] # [doc (hidden)] pub fn supervise_child (child : & mut ChildWrapper , timeout_ms : u64) { if timeout_ms > 0 { wait_timeout (child , timeout_ms) } else { let status = child . wait () . expect ("failed to wait for child") ; assert ! (status . success () , "child exited unsuccessfully with {}" , status) ; } }
};
}
