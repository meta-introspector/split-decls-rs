macro_rules! DocAliasMalformed {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_malformed)] pub (crate) struct DocAliasMalformed { # [primary_span] pub span : Span , }
    };
}

DocAliasMalformed!()