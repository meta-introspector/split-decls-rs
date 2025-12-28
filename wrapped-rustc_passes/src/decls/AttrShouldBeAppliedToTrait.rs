macro_rules! AttrShouldBeAppliedToTrait {
    () => {
        # [derive (Diagnostic)] # [diag (passes_should_be_applied_to_trait)] pub (crate) struct AttrShouldBeAppliedToTrait { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
    };
}

AttrShouldBeAppliedToTrait!();