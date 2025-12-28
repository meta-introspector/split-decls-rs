macro_rules! CouldNotEvalConstPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_could_not_eval_const_pattern)] pub (crate) struct CouldNotEvalConstPattern { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

CouldNotEvalConstPattern!()