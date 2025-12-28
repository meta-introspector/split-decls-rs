macro_rules! DocTestUnknownInclude {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_include)] pub (crate) struct DocTestUnknownInclude { pub path : String , pub value : String , pub inner : & 'static str , # [suggestion (code = "#{inner}[doc = include_str!(\"{value}\")]")] pub sugg : (Span , Applicability) , }
    };
}

DocTestUnknownInclude!();