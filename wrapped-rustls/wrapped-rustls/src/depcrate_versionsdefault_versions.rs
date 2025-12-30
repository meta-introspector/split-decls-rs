// Generated macro for DEFAULT_VERSIONS (static)
macro_rules! Depcrate_versionsDEFAULT_VERSIONS {
() => {
// Module: crate::versions
// Provides: {"DEFAULT_VERSIONS"}
// Dependencies: {}
# [doc = " The version configuration that an application should use by default."] # [doc = ""] # [doc = " This will be [`ALL_VERSIONS`] for now, but gives space in the future"] # [doc = " to remove a version from here and require users to opt-in to older"] # [doc = " versions."] pub static DEFAULT_VERSIONS : & [& SupportedProtocolVersion] = ALL_VERSIONS ;
};
}
