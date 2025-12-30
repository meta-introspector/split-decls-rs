// Generated macro for Rs (struct)
macro_rules! Depcrate_ci_rsRs {
() => {
// Module: crate::ci::rs
// Provides: {"Rs"}
// Dependencies: {}
# [doc = " Work with hermit-rs images"] # [derive (Args)] pub struct Rs { # [command (flatten)] pub cargo_build : CargoBuild , # [doc = " Package to build (see `cargo help pkgid`)"] # [arg (short , long , id = "SPEC")] pub package : String , # [doc = " Create multiple vCPUs."] # [arg (long , default_value_t = 1)] pub smp : usize , # [command (subcommand)] action : Action , }
};
}
