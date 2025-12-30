// Generated macro for build_artifact_lookup (function)
macro_rules! Depcrate_diffbuild_artifact_lookup {
() => {
// Module: crate::diff
// Provides: {"build_artifact_lookup"}
// Dependencies: {}
fn build_artifact_lookup (artifact_sizes : & [ArtifactSize]) -> FxHashMap < & str , usize > { let mut lookup = FxHashMap :: with_capacity_and_hasher (artifact_sizes . len () , Default :: default ()) ; for (i , data) in artifact_sizes . iter () . enumerate () { lookup . insert (& data . label [..] , i) ; } lookup }
};
}
