// Generated macro for build_query_lookup (function)
macro_rules! Depcrate_diffbuild_query_lookup {
() => {
// Module: crate::diff
// Provides: {"build_query_lookup"}
// Dependencies: {}
fn build_query_lookup (query_data : & [QueryData]) -> FxHashMap < & str , usize > { let mut lookup = FxHashMap :: with_capacity_and_hasher (query_data . len () , Default :: default ()) ; for (i , data) in query_data . iter () . enumerate () { lookup . insert (& data . label [..] , i) ; } lookup }
};
}
