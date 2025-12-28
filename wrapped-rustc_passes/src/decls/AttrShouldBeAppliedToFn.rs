macro_rules! AttrShouldBeAppliedToFn {
    () => {
        # [derive (Diagnostic)] # [diag (passes_should_be_applied_to_fn)] pub (crate) struct AttrShouldBeAppliedToFn { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , pub on_crate : bool , }
    };
}

AttrShouldBeAppliedToFn!();