// Generated macro for FileNameEmbeddablePreference (enum)
macro_rules! DepcrateFileNameEmbeddablePreference {
() => {
// Module: crate
// Provides: {"FileNameEmbeddablePreference"}
// Dependencies: {}
# [derive (Clone , Copy , Eq , PartialEq , Hash , Debug)] pub enum FileNameEmbeddablePreference { # [doc = " If a remapped path is available, only embed the `virtual_path` and omit the `local_path`."] # [doc = ""] # [doc = " Otherwise embed the local-path into the `virtual_path`."] RemappedOnly , # [doc = " Embed the original path as well as its remapped `virtual_path` component if available."] LocalAndRemapped , }
};
}
