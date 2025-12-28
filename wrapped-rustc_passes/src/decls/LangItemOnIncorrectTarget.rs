macro_rules! LangItemOnIncorrectTarget {
    () => {
        # [derive (Diagnostic)] # [diag (passes_lang_item_on_incorrect_target , code = E0718)] pub (crate) struct LangItemOnIncorrectTarget { # [primary_span] # [label] pub span : Span , pub name : Symbol , pub expected_target : Target , pub actual_target : Target , }
    };
}

LangItemOnIncorrectTarget!()