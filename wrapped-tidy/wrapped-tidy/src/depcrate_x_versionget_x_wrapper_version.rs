// Generated macro for get_x_wrapper_version (function)
macro_rules! Depcrate_x_versionget_x_wrapper_version {
() => {
// Module: crate::x_version
// Provides: {"get_x_wrapper_version"}
// Dependencies: {}
fn get_x_wrapper_version (root : & Path , cargo : & Path) -> Option < Version > { let mut cmd = cargo_metadata :: MetadataCommand :: new () ; cmd . cargo_path (cargo) . manifest_path (root . join ("src/tools/x/Cargo.toml")) . no_deps () . features (cargo_metadata :: CargoOpt :: AllFeatures) ; let mut metadata = t ! (cmd . exec ()) ; metadata . packages . pop () . map (| x | x . version) }
};
}
