macro_rules! UnstableAttrForAlreadyStableFeature {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unstable_attr_for_already_stable_feature)] pub (crate) struct UnstableAttrForAlreadyStableFeature { # [primary_span] # [label] # [help] pub attr_span : Span , # [label (passes_item)] pub item_span : Span , }
    };
}

UnstableAttrForAlreadyStableFeature!();