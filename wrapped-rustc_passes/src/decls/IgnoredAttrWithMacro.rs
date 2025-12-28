macro_rules! IgnoredAttrWithMacro {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_ignored_attr_with_macro)] pub (crate) struct IgnoredAttrWithMacro < 'a > { pub sym : & 'a str , }
    };
}

IgnoredAttrWithMacro!()