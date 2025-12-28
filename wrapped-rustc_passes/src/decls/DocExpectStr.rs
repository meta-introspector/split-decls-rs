macro_rules! DocExpectStr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_expect_str)] pub (crate) struct DocExpectStr < 'a > { # [primary_span] pub attr_span : Span , pub attr_name : & 'a str , }
    };
}

DocExpectStr!();