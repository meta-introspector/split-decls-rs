// Generated macro for compute_alias_components_recursive (function)
macro_rules! Depcrate_outlivescompute_alias_components_recursive {
() => {
// Module: crate::outlives
// Provides: {"compute_alias_components_recursive"}
// Dependencies: {}
# [doc = " Collect [Component]s for *all* the args of `alias_ty`."] # [doc = ""] # [doc = " This should not be used to get the components of `alias_ty` itself."] # [doc = " Use [push_outlives_components] instead."] pub fn compute_alias_components_recursive < I : Interner > (cx : I , kind : ty :: AliasTyKind , alias_ty : ty :: AliasTy < I > , out : & mut SmallVec < [Component < I > ; 4] > ,) { let opt_variances = cx . opt_alias_variances (kind , alias_ty . def_id) ; let mut visitor = OutlivesCollector { cx , out , visited : Default :: default () } ; for (index , child) in alias_ty . args . iter () . enumerate () { if opt_variances . and_then (| variances | variances . get (index)) == Some (ty :: Bivariant) { continue ; } child . visit_with (& mut visitor) ; } }
};
}
