macro_rules! DocAttrNotCrateLevel {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_attr_not_crate_level)] pub (crate) struct DocAttrNotCrateLevel < 'a > { # [primary_span] pub span : Span , pub attr_name : & 'a str , }
    };
}

DocAttrNotCrateLevel!()