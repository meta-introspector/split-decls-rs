macro_rules! DocKeywordAttributeEmptyMod {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_keyword_attribute_empty_mod)] pub (crate) struct DocKeywordAttributeEmptyMod { # [primary_span] pub span : Span , pub attr_name : & 'static str , }
    };
}

DocKeywordAttributeEmptyMod!()