// Generated macro for impl_282 (impl)
macro_rules! Depcrate_symbolimpl_282 {
() => {
// Module: crate::symbol
// Provides: {"impl_282"}
// Dependencies: {}
impl StableCompare for Symbol { const CAN_USE_UNSTABLE_SORT : bool = true ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
};
}
