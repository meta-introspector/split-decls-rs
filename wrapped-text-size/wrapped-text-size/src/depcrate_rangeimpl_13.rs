// Generated macro for impl_13 (impl)
macro_rules! Depcrate_rangeimpl_13 {
() => {
// Module: crate::range
// Provides: {"impl_13"}
// Dependencies: {}
impl RangeBounds < TextSize > for TextRange { fn start_bound (& self) -> Bound < & TextSize > { Bound :: Included (& self . start) } fn end_bound (& self) -> Bound < & TextSize > { Bound :: Excluded (& self . end) } }
};
}
