macro_rules! DocKeywordAttributeNotMod {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_keyword_attribute_not_mod)] pub (crate) struct DocKeywordAttributeNotMod { # [primary_span] pub span : Span , pub attr_name : & 'static str , }
    };
}

DocKeywordAttributeNotMod!();