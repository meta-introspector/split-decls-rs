macro_rules! DocAttributeNotAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_attribute_not_attribute)] # [help] pub (crate) struct DocAttributeNotAttribute { # [primary_span] pub span : Span , pub attribute : Symbol , }
    };
}

DocAttributeNotAttribute!()