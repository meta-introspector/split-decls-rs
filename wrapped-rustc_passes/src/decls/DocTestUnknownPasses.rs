macro_rules! DocTestUnknownPasses {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_passes)] # [note] # [help] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownPasses { pub path : String , # [label] pub span : Span , }
    };
}

DocTestUnknownPasses!();