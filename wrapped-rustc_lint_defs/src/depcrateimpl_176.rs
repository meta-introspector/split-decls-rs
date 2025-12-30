// Generated macro for impl_176 (impl)
macro_rules! Depcrateimpl_176 {
() => {
// Module: crate
// Provides: {"impl_176"}
// Dependencies: {}
impl StableCompare for LintId { const CAN_USE_UNSTABLE_SORT : bool = true ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . lint_name_raw () . cmp (& other . lint_name_raw ()) } }
};
}
