macro_rules! DocAliasDuplicated {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_alias_duplicated)] pub (crate) struct DocAliasDuplicated { # [label] pub first_defn : Span , }
    };
}

DocAliasDuplicated!();