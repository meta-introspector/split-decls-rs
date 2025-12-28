macro_rules! DocAliasNotAnAlias {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_not_an_alias)] pub (crate) struct DocAliasNotAnAlias < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }
    };
}

DocAliasNotAnAlias!();