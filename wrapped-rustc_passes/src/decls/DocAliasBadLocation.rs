macro_rules! DocAliasBadLocation {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_bad_location)] pub (crate) struct DocAliasBadLocation < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub location : & 'a str , }
    };
}

DocAliasBadLocation!();