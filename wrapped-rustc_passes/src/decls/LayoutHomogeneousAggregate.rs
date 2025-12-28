macro_rules! LayoutHomogeneousAggregate {
    () => {
        # [derive (Diagnostic)] # [diag (passes_layout_homogeneous_aggregate)] pub (crate) struct LayoutHomogeneousAggregate { # [primary_span] pub span : Span , pub homogeneous_aggregate : String , }
    };
}

LayoutHomogeneousAggregate!();