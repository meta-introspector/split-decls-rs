// Generated macro for Artifact (struct)
macro_rules! Depcrate_artifactArtifact {
() => {
// Module: crate::artifact
// Provides: {"Artifact"}
// Dependencies: {}
# [derive (Args)] pub struct Artifact { # [doc = " Target architecture."] # [arg (value_enum , long)] pub arch : Arch , # [doc = " Directory for all generated artifacts."] # [arg (long , id = "DIRECTORY")] pub target_dir : Option < PathBuf > , # [doc = " Copy final artifacts to this directory"] # [arg (long , id = "PATH")] pub artifact_dir : Option < PathBuf > , # [doc = " Build artifacts in release mode, with optimizations."] # [arg (short , long)] pub release : bool , # [doc = " Build artifacts with the specified profile."] # [arg (long , id = "PROFILE-NAME")] pub profile : Option < String > , }
};
}
