// Generated macro for impl_64 (impl)
macro_rules! Depcrate_diffimpl_64 {
() => {
// Module: crate::diff
// Provides: {"impl_64"}
// Dependencies: {}
impl ArtifactSizeDiff { pub fn invert_artifact_size (size : & ArtifactSize) -> ArtifactSizeDiff { ArtifactSizeDiff { label : size . label . clone () , size_change : - (size . value as i64) , } } pub fn artifact_size_as_diff (size : & ArtifactSize) -> ArtifactSizeDiff { ArtifactSizeDiff { label : size . label . clone () , size_change : size . value as i64 , } } fn sub (lhs : ArtifactSize , rhs : ArtifactSize) -> ArtifactSizeDiff { ArtifactSizeDiff { label : lhs . label , size_change : lhs . value as i64 - rhs . value as i64 , } } }
};
}
