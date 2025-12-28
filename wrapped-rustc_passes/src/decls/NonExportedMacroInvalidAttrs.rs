macro_rules! NonExportedMacroInvalidAttrs {
    () => {
        # [derive (Diagnostic)] # [diag (passes_non_exported_macro_invalid_attrs , code = E0518)] pub (crate) struct NonExportedMacroInvalidAttrs { # [primary_span] # [label] pub attr_span : Span , }
    };
}

NonExportedMacroInvalidAttrs!();