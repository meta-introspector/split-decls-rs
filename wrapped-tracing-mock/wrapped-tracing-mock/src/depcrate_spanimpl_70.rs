// Generated macro for impl_70 (impl)
macro_rules! Depcrate_spanimpl_70 {
() => {
// Module: crate::span
// Provides: {"impl_70"}
// Dependencies: {}
impl < S > From < S > for NewSpan where S : Into < ExpectedSpan > , { fn from (span : S) -> Self { Self { span : span . into () , .. Default :: default () } } }
};
}
