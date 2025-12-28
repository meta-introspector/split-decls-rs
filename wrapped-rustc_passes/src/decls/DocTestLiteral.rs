macro_rules! DocTestLiteral {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_literal)] pub (crate) struct DocTestLiteral ;
    };
}

DocTestLiteral!()