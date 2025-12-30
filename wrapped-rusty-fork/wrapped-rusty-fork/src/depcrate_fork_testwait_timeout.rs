// Generated macro for wait_timeout (function)
macro_rules! Depcrate_fork_testwait_timeout {
() => {
// Module: crate::fork_test
// Provides: {"wait_timeout"}
// Dependencies: {}
# [cfg (not (feature = "timeout"))] fn wait_timeout (_ : & mut ChildWrapper , _ : u64) { panic ! ("Using the timeout feature of rusty_fork_test! requires \
            enabling the `timeout` feature on the rusty-fork crate.") ; }
};
}
