macro_rules! DocMaskedNotExternCrateSelf {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_masked_not_extern_crate_self)] pub (crate) struct DocMaskedNotExternCrateSelf { # [label] pub attr_span : Span , # [label (passes_extern_crate_self_label)] pub item_span : Option < Span > , }
    };
}

DocMaskedNotExternCrateSelf!()