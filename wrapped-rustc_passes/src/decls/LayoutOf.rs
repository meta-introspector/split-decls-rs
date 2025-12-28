macro_rules! LayoutOf {
    () => {
        # [derive (Diagnostic)] # [diag (passes_layout_of)] pub (crate) struct LayoutOf < 'tcx > { # [primary_span] pub span : Span , pub normalized_ty : Ty < 'tcx > , pub ty_layout : String , }
    };
}

LayoutOf!()