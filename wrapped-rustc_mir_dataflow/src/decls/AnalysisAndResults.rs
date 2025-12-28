macro_rules! deps {
    () => {
        Results!();
        Analysis!();
    };
}

macro_rules! AnalysisAndResults {
    () => {
        deps!();
        # [doc = " Utility type used in a few places where it's convenient to bundle an analysis with its results."] pub struct AnalysisAndResults < 'tcx , A > where A : Analysis < 'tcx > , { pub analysis : A , pub results : Results < A :: Domain > , }
    };
}

AnalysisAndResults!();