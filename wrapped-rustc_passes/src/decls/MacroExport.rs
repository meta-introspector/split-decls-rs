macro_rules! MacroExport {
    () => {
        # [derive (LintDiagnostic)] pub (crate) enum MacroExport { # [diag (passes_macro_export)] Normal , # [diag (passes_macro_export_on_decl_macro)] # [note] OnDeclMacro , # [diag (passes_invalid_macro_export_arguments)] InvalidArgument , # [diag (passes_invalid_macro_export_arguments_too_many_items)] TooManyItems , }
    };
}

MacroExport!();