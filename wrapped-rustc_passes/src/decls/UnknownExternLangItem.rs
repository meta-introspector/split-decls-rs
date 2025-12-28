macro_rules! UnknownExternLangItem {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unknown_external_lang_item , code = E0264)] pub (crate) struct UnknownExternLangItem { # [primary_span] pub span : Span , pub lang_item : Symbol , }
    };
}

UnknownExternLangItem!()