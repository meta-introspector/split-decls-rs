macro_rules! deps {
    () => {
        ImplConstStability!();
        TraitConstStability!();
    };
}

macro_rules! TraitImplConstStabilityMismatch {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (passes_trait_impl_const_stability_mismatch)] pub (crate) struct TraitImplConstStabilityMismatch { # [primary_span] pub span : Span , # [subdiagnostic] pub impl_stability : ImplConstStability , # [subdiagnostic] pub trait_stability : TraitConstStability , }
    };
}

TraitImplConstStabilityMismatch!();