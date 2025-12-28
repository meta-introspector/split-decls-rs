macro_rules! DocAliasNotStringLiteral {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_alias_not_string_literal)] pub (crate) struct DocAliasNotStringLiteral { # [primary_span] pub span : Span , }
    };
}

DocAliasNotStringLiteral!()