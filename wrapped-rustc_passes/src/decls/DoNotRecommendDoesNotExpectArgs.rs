macro_rules! DoNotRecommendDoesNotExpectArgs {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_incorrect_do_not_recommend_args)] pub (crate) struct DoNotRecommendDoesNotExpectArgs ;
    };
}

DoNotRecommendDoesNotExpectArgs!();