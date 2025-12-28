macro_rules! IncorrectCrateType {
    () => {
        # [derive (Diagnostic)] # [diag (passes_incorrect_crate_type)] pub (crate) struct IncorrectCrateType { # [primary_span] pub span : Span , }
    };
}

IncorrectCrateType!();