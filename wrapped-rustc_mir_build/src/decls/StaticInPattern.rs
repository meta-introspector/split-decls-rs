macro_rules! StaticInPattern {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_static_in_pattern , code = E0158)] pub (crate) struct StaticInPattern { # [primary_span] # [label] pub (crate) span : Span , # [label (mir_build_static_in_pattern_def)] pub (crate) static_span : Span , }
    };
}

StaticInPattern!()