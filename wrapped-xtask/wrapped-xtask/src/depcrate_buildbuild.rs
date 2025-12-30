// Generated macro for Build (struct)
macro_rules! Depcrate_buildBuild {
() => {
// Module: crate::build
// Provides: {"Build"}
// Dependencies: {}
# [doc = " Build the kernel."] # [derive (Args)] pub struct Build { # [command (flatten)] cargo_build : CargoBuild , # [doc = " Enable the `-Z instrument-mcount` flag."] # [arg (long)] pub instrument_mcount : bool , # [doc = " Enable the `-Z randomize-layout` flag."] # [arg (long)] pub randomize_layout : bool , }
};
}
