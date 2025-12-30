// Generated macro for pat_is_catchall (function)
macro_rules! Depcrate_thir_pattern_check_matchpat_is_catchall {
() => {
// Module: crate::thir::pattern::check_match
// Provides: {"pat_is_catchall"}
// Dependencies: {}
# [doc = " Checks for common cases of \"catchall\" patterns that may not be intended as such."] fn pat_is_catchall (pat : & DeconstructedPat < '_ , '_ >) -> bool { match pat . ctor () { Constructor :: Wildcard => true , Constructor :: Struct | Constructor :: Ref => { pat . iter_fields () . all (| ipat | pat_is_catchall (& ipat . pat)) } _ => false , } }
};
}
