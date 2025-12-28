macro_rules! AttrOnlyInFunctions {
    () => {
        # [derive (Diagnostic)] # [diag (passes_attr_only_in_functions)] pub (crate) struct AttrOnlyInFunctions { # [primary_span] pub span : Span , pub attr : Symbol , }
    };
}

AttrOnlyInFunctions!()