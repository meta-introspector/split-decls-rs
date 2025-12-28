macro_rules! ReprAlignShouldBeAlignStatic {
    () => {
        # [derive (Diagnostic)] # [diag (passes_repr_align_should_be_align_static)] pub (crate) struct ReprAlignShouldBeAlignStatic { # [primary_span] # [help] pub span : Span , pub item : & 'static str , }
    };
}

ReprAlignShouldBeAlignStatic!()