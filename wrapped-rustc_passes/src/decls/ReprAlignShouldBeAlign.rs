macro_rules! ReprAlignShouldBeAlign {
    () => {
        # [derive (Diagnostic)] # [diag (passes_repr_align_should_be_align)] pub (crate) struct ReprAlignShouldBeAlign { # [primary_span] # [help] pub span : Span , pub item : & 'static str , }
    };
}

ReprAlignShouldBeAlign!();