macro_rules! LangItemWithTargetFeature {
    () => {
        # [derive (Diagnostic)] # [diag (passes_lang_item_fn_with_target_feature)] pub (crate) struct LangItemWithTargetFeature { # [primary_span] pub attr_span : Span , pub name : Symbol , # [label] pub sig_span : Span , }
    };
}

LangItemWithTargetFeature!();