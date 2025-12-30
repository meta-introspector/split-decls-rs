// Generated macro for proc_selector_to_raw (function)
macro_rules! Depcrate_process_procctlproc_selector_to_raw {
() => {
// Module: crate::process::procctl
// Provides: {"proc_selector_to_raw"}
// Dependencies: {}
fn proc_selector_to_raw (selector : ProcSelector) -> (IdType , RawPid) { match selector { Some ((idtype , id)) => (idtype , id . as_raw_nonzero () . get ()) , None => (IdType :: Pid , 0) , } }
};
}
