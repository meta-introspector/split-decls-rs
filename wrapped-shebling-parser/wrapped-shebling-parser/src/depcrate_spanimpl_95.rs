// Generated macro for impl_95 (impl)
macro_rules! Depcrate_spanimpl_95 {
() => {
// Module: crate::span
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a > nom :: Compare < & str > for ParseSpan < 'a > { # [inline (always)] fn compare (& self , t : & str) -> nom :: CompareResult { self . fragment . compare (t) } fn compare_no_case (& self , t : & str) -> nom :: CompareResult { self . fragment . compare_no_case (t) } }
};
}
