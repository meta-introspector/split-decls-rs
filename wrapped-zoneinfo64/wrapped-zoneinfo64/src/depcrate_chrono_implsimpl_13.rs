// Generated macro for impl_13 (impl)
macro_rules! Depcrate_chrono_implsimpl_13 {
() => {
// Module: crate::chrono_impls
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > chrono :: Offset for ChronoOffset < 'a > { fn fix (& self) -> FixedOffset { FixedOffset :: east_opt (self . 0 . offset . 0) . unwrap () } }
};
}
