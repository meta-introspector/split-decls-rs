// Generated macro for impl_162 (impl)
macro_rules! Depcrate_framework_resultsimpl_162 {
() => {
// Module: crate::framework::results
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'tcx , A > AnalysisAndResults < 'tcx , A > where A : Analysis < 'tcx > , { # [doc = " Creates a `ResultsCursor` that takes ownership of `self`."] pub fn into_results_cursor < 'mir > (self , body : & 'mir Body < 'tcx >) -> ResultsCursor < 'mir , 'tcx , A > { ResultsCursor :: new_owning (body , self . analysis , self . results) } }
};
}
