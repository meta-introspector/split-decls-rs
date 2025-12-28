macro_rules! DocKeywordNotKeyword {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_keyword_not_keyword)] # [help] pub (crate) struct DocKeywordNotKeyword { # [primary_span] pub span : Span , pub keyword : Symbol , }
    };
}

DocKeywordNotKeyword!()