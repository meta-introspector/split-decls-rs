macro_rules! ReprConflicting {
    () => {
        # [derive (Diagnostic)] # [diag (passes_repr_conflicting , code = E0566)] pub (crate) struct ReprConflicting { # [primary_span] pub hint_spans : Vec < Span > , }
    };
}

ReprConflicting!()