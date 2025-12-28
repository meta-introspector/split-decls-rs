macro_rules! DocTestUnknownPlugins {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_plugins)] # [note] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownPlugins { pub path : String , # [label] pub span : Span , }
    };
}

DocTestUnknownPlugins!()