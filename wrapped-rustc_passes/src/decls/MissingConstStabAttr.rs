macro_rules! MissingConstStabAttr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_missing_const_stab_attr)] pub (crate) struct MissingConstStabAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }
    };
}

MissingConstStabAttr!()