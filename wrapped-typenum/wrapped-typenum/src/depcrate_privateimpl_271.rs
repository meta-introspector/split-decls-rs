// Generated macro for impl_271 (impl)
macro_rules! Depcrate_privateimpl_271 {
() => {
// Module: crate::private
// Provides: {"impl_271"}
// Dependencies: {}
impl < U : Unsigned > Trim for U where U : Invert , < U as Invert > :: Output : TrimTrailingZeros , < < U as Invert > :: Output as TrimTrailingZeros > :: Output : Invert , { type Output = < < < U as Invert > :: Output as TrimTrailingZeros > :: Output as Invert > :: Output ; # [inline] fn trim (self) -> Self :: Output { self . invert () . trim_trailing_zeros () . invert () } }
};
}
