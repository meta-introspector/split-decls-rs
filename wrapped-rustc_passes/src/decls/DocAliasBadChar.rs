macro_rules! DocAliasBadChar {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_bad_char)] pub (crate) struct DocAliasBadChar < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub char_ : char , }
    };
}

DocAliasBadChar!();