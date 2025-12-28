macro_rules! CustomMirPhaseRequiresDialect {
    () => {
        # [derive (Diagnostic)] # [diag (passes_custom_mir_phase_requires_dialect)] pub (crate) struct CustomMirPhaseRequiresDialect { # [primary_span] pub attr_span : Span , # [label] pub phase_span : Span , }
    };
}

CustomMirPhaseRequiresDialect!();