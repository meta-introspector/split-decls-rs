// Generated macro for CargoBuild (struct)
macro_rules! Depcrate_cargo_buildCargoBuild {
() => {
// Module: crate::cargo_build
// Provides: {"CargoBuild"}
// Dependencies: {}
# [derive (Args)] pub struct CargoBuild { # [command (flatten)] pub artifact : Artifact , # [doc = " Do not activate the `default` feature."] # [arg (long)] no_default_features : bool , # [doc = " Space or comma separated list of features to activate."] # [arg (long)] pub features : Vec < String > , }
};
}
