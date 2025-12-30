// Generated macro for impl_1442 (impl)
macro_rules! Depcrate_serverimpl_1442 {
() => {
// Module: crate::server
// Provides: {"impl_1442"}
// Dependencies: {}
impl WaitUntilZero { # [rustfmt :: skip] fn new () -> (WaitUntilZero , ActiveInfo) { let info = Arc :: new (std :: sync :: Mutex :: new (Info { waker : None })) ; (WaitUntilZero { info : Arc :: downgrade (& info) } , ActiveInfo { info }) } }
};
}
