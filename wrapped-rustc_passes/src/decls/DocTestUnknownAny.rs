macro_rules! DocTestUnknownAny {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_any)] pub (crate) struct DocTestUnknownAny { pub path : String , }
    };
}

DocTestUnknownAny!()