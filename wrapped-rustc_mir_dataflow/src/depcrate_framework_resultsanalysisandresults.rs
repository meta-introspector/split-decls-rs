// Generated macro for AnalysisAndResults (struct)
macro_rules! Depcrate_framework_resultsAnalysisAndResults {
() => {
// Module: crate::framework::results
// Provides: {"AnalysisAndResults"}
// Dependencies: {}
# [doc = " Utility type used in a few places where it's convenient to bundle an analysis with its results."] pub struct AnalysisAndResults < 'tcx , A > where A : Analysis < 'tcx > , { pub analysis : A , pub results : Results < A :: Domain > , }
};
}
