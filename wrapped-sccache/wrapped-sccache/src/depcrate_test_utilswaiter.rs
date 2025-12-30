// Generated macro for Waiter (trait)
macro_rules! Depcrate_test_utilsWaiter {
() => {
// Module: crate::test::utils
// Provides: {"Waiter"}
// Dependencies: {}
# [doc = " An add on trait, to allow calling `.wait()` for `futures::Future`"] # [doc = " as it was possible for `futures` at `0.1`."] # [doc = ""] # [doc = " Intended for test only!"] pub (crate) trait Waiter < R > { fn wait (self) -> R ; }
};
}
