// Generated macro for impl_51 (impl)
macro_rules! Depcrate_test_utilsimpl_51 {
() => {
// Module: crate::test::utils
// Provides: {"impl_51"}
// Dependencies: {}
impl < T , O > Waiter < O > for T where T : Future < Output = O > , { fn wait (self) -> O { let rt = single_threaded_runtime () ; rt . block_on (self) } }
};
}
