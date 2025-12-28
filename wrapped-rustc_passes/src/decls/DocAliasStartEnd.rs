macro_rules! DocAliasStartEnd {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_start_end)] pub (crate) struct DocAliasStartEnd < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }
    };
}

DocAliasStartEnd!()