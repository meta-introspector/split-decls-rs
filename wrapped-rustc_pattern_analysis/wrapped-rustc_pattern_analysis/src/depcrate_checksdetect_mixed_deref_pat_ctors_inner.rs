// Generated macro for detect_mixed_deref_pat_ctors_inner (function)
macro_rules! Depcrate_checksdetect_mixed_deref_pat_ctors_inner {
() => {
// Module: crate::checks
// Provides: {"detect_mixed_deref_pat_ctors_inner"}
// Dependencies: {}
fn detect_mixed_deref_pat_ctors_inner < 'p , Cx : PatCx > (cx : & Cx , column : & PatternColumn < 'p , Cx > ,) -> Result < () , Cx :: Error > { let Some (ty) = column . head_ty () else { return Ok (()) ; } ; let mut deref_pat = None ; let mut normal_pat = None ; for pat in column . iter () { match pat . ctor () { Wildcard | Opaque (_) => { } DerefPattern (_) => deref_pat = Some (pat) , _ => normal_pat = Some (pat) , } } if let Some (deref_pat) = deref_pat && let Some (normal_pat) = normal_pat { return Err (cx . report_mixed_deref_pat_ctors (deref_pat , normal_pat)) ; } let set = column . analyze_ctors (cx , & ty) ? ; for ctor in set . present { for specialized_column in column . specialize (cx , & ty , & ctor) . iter () { detect_mixed_deref_pat_ctors_inner (cx , specialized_column) ? ; } } Ok (()) }
};
}
