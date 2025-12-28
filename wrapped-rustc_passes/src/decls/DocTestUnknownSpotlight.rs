macro_rules! DocTestUnknownSpotlight {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_spotlight)] # [note] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownSpotlight { pub path : String , # [suggestion (style = "short" , applicability = "machine-applicable" , code = "notable_trait")] pub span : Span , }
    };
}

DocTestUnknownSpotlight!()