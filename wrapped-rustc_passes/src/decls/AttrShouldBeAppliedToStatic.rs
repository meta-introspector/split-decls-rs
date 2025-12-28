macro_rules! AttrShouldBeAppliedToStatic {
    () => {
        # [derive (Diagnostic)] # [diag (passes_should_be_applied_to_static)] pub (crate) struct AttrShouldBeAppliedToStatic { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
    };
}

AttrShouldBeAppliedToStatic!();