// Generated macro for CommandEnvs (struct)
macro_rules! Depcrate_processCommandEnvs {
() => {
// Module: crate::process
// Provides: {"CommandEnvs"}
// Dependencies: {}
# [doc = " An iterator over the command environment variables."] # [doc = ""] # [doc = " This struct is created by"] # [doc = " [`Command::get_envs`][crate::process::Command::get_envs]. See its"] # [doc = " documentation for more."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "command_access" , since = "1.57.0")] pub struct CommandEnvs < 'a > { iter : imp :: CommandEnvs < 'a > , }
};
}
