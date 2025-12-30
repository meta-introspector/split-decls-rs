// Generated macro for IdType (enum)
macro_rules! Depcrate_process_procctlIdType {
() => {
// Module: crate::process::procctl
// Provides: {"IdType"}
// Dependencies: {}
# [doc = " Subset of `idtype_t` C enum, with only the values allowed by `procctl`."] # [repr (i32)] pub enum IdType { # [doc = " Process id."] Pid = 0 , # [doc = " Process group id."] Pgid = 2 , }
};
}
