// Generated macro for detect_mixed_deref_pat_ctors (function)
macro_rules! Depcrate_checksdetect_mixed_deref_pat_ctors {
() => {
// Module: crate::checks
// Provides: {"detect_mixed_deref_pat_ctors"}
// Dependencies: {}
# [doc = " Validate that deref patterns and normal constructors aren't used to match on the same place."] pub (crate) fn detect_mixed_deref_pat_ctors < 'p , Cx : PatCx > (cx : & Cx , arms : & [MatchArm < 'p , Cx >] ,) -> Result < () , Cx :: Error > { let pat_column = PatternColumn :: new (arms) ; detect_mixed_deref_pat_ctors_inner (cx , & pat_column) }
};
}
