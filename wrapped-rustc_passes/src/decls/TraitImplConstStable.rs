macro_rules! TraitImplConstStable {
    () => {
        # [derive (Diagnostic)] # [diag (passes_trait_impl_const_stable)] # [note] pub (crate) struct TraitImplConstStable { # [primary_span] pub span : Span , }
    };
}

TraitImplConstStable!()