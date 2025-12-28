macro_rules! DocMaskedOnlyExternCrate {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_masked_only_extern_crate)] # [note] pub (crate) struct DocMaskedOnlyExternCrate { # [label] pub attr_span : Span , # [label (passes_not_an_extern_crate_label)] pub item_span : Option < Span > , }
    };
}

DocMaskedOnlyExternCrate!();