macro_rules! DocTestUnknown {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown)] pub (crate) struct DocTestUnknown { pub path : String , }
    };
}

DocTestUnknown!()