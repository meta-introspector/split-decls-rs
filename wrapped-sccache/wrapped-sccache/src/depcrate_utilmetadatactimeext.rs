// Generated macro for MetadataCtimeExt (trait)
macro_rules! Depcrate_utilMetadataCtimeExt {
() => {
// Module: crate::util
// Provides: {"MetadataCtimeExt"}
// Dependencies: {}
# [doc = " Adds a fallback for trying Unix's `ctime` semantics on Windows systems."] pub trait MetadataCtimeExt { fn ctime_or_creation (& self) -> std :: io :: Result < Timestamp > ; }
};
}
