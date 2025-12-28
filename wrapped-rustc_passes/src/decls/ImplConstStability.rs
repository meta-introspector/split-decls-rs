macro_rules! ImplConstStability {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ImplConstStability { # [note (passes_trait_impl_const_stability_mismatch_impl_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_impl_unstable)] Unstable { # [primary_span] span : Span , } , }
    };
}

ImplConstStability!();