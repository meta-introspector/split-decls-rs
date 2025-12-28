macro_rules! DocAliasEmpty {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_empty)] pub (crate) struct DocAliasEmpty < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }
    };
}

DocAliasEmpty!()