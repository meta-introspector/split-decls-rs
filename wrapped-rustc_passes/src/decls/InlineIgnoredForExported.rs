macro_rules! InlineIgnoredForExported {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_inline_ignored_for_exported)] # [help] pub (crate) struct InlineIgnoredForExported { }
    };
}

InlineIgnoredForExported!()