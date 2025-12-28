macro_rules! DocInlineOnlyUse {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_doc_inline_only_use)] # [note] pub (crate) struct DocInlineOnlyUse { # [label] pub attr_span : Span , # [label (passes_not_a_use_item_label)] pub item_span : Option < Span > , }
    };
}

DocInlineOnlyUse!();