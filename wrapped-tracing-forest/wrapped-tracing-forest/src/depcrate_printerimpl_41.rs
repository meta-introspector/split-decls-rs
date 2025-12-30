// Generated macro for impl_41 (impl)
macro_rules! Depcrate_printerimpl_41 {
() => {
// Module: crate::printer
// Provides: {"impl_41"}
// Dependencies: {}
impl TestCapturePrinter < Pretty > { # [doc = " Construct a new test capturing printer with the default `Pretty` formatter. This printer"] # [doc = " is intented for use in tests only as it works with the default rust stdout capture mechanism"] # [allow (clippy :: new_without_default)] pub const fn new () -> Self { TestCapturePrinter { formatter : Pretty } } }
};
}
