macro_rules! ConstParamInPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_const_param_in_pattern , code = E0158)] pub (crate) struct ConstParamInPattern { # [primary_span] # [label] pub (crate) span : Span , # [label (mir_build_const_param_in_pattern_def)] pub (crate) const_span : Span , }
    };
}

ConstParamInPattern!();