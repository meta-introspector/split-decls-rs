macro_rules! DocKeywordOnlyImpl {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_keyword_only_impl)] pub (crate) struct DocKeywordOnlyImpl { # [primary_span] pub span : Span , }
    };
}

DocKeywordOnlyImpl!();