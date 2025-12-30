// Generated macro for impl_261 (impl)
macro_rules! Depcrate_privateimpl_261 {
() => {
// Module: crate::private
// Provides: {"impl_261"}
// Dependencies: {}
impl < IU : InvertedUnsigned > TrimTrailingZeros for InvertedUInt < IU , B0 > where IU : TrimTrailingZeros , { type Output = < IU as TrimTrailingZeros > :: Output ; # [inline] fn trim_trailing_zeros (self) -> Self :: Output { self . msb . trim_trailing_zeros () } }
};
}
