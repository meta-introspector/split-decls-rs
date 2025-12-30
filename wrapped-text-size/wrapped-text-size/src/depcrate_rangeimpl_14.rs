// Generated macro for impl_14 (impl)
macro_rules! Depcrate_rangeimpl_14 {
() => {
// Module: crate::range
// Provides: {"impl_14"}
// Dependencies: {}
impl < T > From < TextRange > for Range < T > where T : From < TextSize > , { # [inline] fn from (r : TextRange) -> Self { r . start () . into () .. r . end () . into () } }
};
}
