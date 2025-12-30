// Generated macro for ToolchainPackager (trait)
macro_rules! Depcrate_dist_pkgToolchainPackager {
() => {
// Module: crate::dist::pkg
// Provides: {"ToolchainPackager"}
// Dependencies: {}
pub trait ToolchainPackager : Send { fn write_pkg (self : Box < Self > , f : fs :: File) -> Result < () > ; }
};
}
