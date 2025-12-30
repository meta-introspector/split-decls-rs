// Generated macro for ArtifactDescriptor (struct)
macro_rules! Depcrate_compiler_cArtifactDescriptor {
() => {
// Module: crate::compiler::c
// Provides: {"ArtifactDescriptor"}
// Dependencies: {}
# [doc = " Artifact produced by a C/C++ compiler."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ArtifactDescriptor { # [doc = " Path to the artifact."] pub path : PathBuf , # [doc = " Whether the artifact is an optional object file."] pub optional : bool , }
};
}
