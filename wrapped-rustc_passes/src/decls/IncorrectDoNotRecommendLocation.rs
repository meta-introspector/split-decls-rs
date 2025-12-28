macro_rules! IncorrectDoNotRecommendLocation {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_incorrect_do_not_recommend_location)] pub (crate) struct IncorrectDoNotRecommendLocation ;
    };
}

IncorrectDoNotRecommendLocation!();