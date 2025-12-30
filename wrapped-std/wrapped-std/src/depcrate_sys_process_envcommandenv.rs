// Generated macro for CommandEnv (struct)
macro_rules! Depcrate_sys_process_envCommandEnv {
() => {
// Module: crate::sys::process::env
// Provides: {"CommandEnv"}
// Dependencies: {}
# [doc = " Stores a set of changes to an environment"] # [derive (Clone , Default)] pub struct CommandEnv { clear : bool , saw_path : bool , vars : BTreeMap < EnvKey , Option < OsString > > , }
};
}
