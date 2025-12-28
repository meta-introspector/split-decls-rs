macro_rules! UnknownLangItem {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unknown_lang_item , code = E0522)] pub (crate) struct UnknownLangItem { # [primary_span] # [label] pub span : Span , pub name : Symbol , }
    };
}

UnknownLangItem!();