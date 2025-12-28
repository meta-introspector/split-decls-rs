macro_rules! DocKeywordConflict {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_inline_conflict)] # [help] pub (crate) struct DocKeywordConflict { # [primary_span] pub spans : MultiSpan , }
    };
}

DocKeywordConflict!()