// Generated macro for C (struct)
macro_rules! Depcrate_ci_cC {
() => {
// Module: crate::ci::c
// Provides: {"C"}
// Dependencies: {}
# [doc = " Work with hermit-c images"] # [derive (Args)] pub struct C { # [doc = " Target architecture."] # [arg (value_enum , long)] pub arch : Arch , # [doc = " Build type to use."] # [arg (long , default_value = "debug")] pub buildtype : String , # [doc = " Target to build."] # [arg (long , id = "SPEC")] pub target : String , # [doc = " Create multiple vCPUs."] # [arg (long , default_value_t = 1)] pub smp : usize , # [command (subcommand)] action : Action , }
};
}
