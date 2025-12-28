macro_rules! DocInvalid {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_invalid)] pub (crate) struct DocInvalid ;
    };
}

DocInvalid!();