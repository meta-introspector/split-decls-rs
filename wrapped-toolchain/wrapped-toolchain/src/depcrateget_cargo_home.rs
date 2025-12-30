// Generated macro for get_cargo_home (function)
macro_rules! Depcrateget_cargo_home {
() => {
// Module: crate
// Provides: {"get_cargo_home"}
// Dependencies: {}
fn get_cargo_home () -> Option < Utf8PathBuf > { if let Some (path) = env :: var_os ("CARGO_HOME") { return Utf8PathBuf :: try_from (PathBuf :: from (path)) . ok () ; } if let Some (mut path) = home :: home_dir () { path . push (".cargo") ; return Utf8PathBuf :: try_from (path) . ok () ; } None }
};
}
