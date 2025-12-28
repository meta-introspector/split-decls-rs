macro_rules! DocSearchUnboxInvalid {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_search_unbox_invalid)] pub (crate) struct DocSearchUnboxInvalid { # [primary_span] pub span : Span , }
    };
}

DocSearchUnboxInvalid!()