// Generated macro for ProcessInner (struct)
macro_rules! Depcrate_unix_freebsd_processProcessInner {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"ProcessInner"}
// Dependencies: {}
pub (crate) struct ProcessInner { pub (crate) name : OsString , pub (crate) cmd : Vec < OsString > , pub (crate) exe : Option < PathBuf > , pub (crate) pid : Pid , parent : Option < Pid > , pub (crate) environ : Vec < OsString > , pub (crate) cwd : Option < PathBuf > , pub (crate) root : Option < PathBuf > , pub (crate) memory : u64 , pub (crate) virtual_memory : u64 , pub (crate) updated : bool , cpu_usage : f32 , start_time : u64 , run_time : u64 , pub (crate) status : ProcessStatus , user_id : Uid , effective_user_id : Uid , group_id : Gid , effective_group_id : Gid , read_bytes : u64 , old_read_bytes : u64 , written_bytes : u64 , old_written_bytes : u64 , accumulated_cpu_time : u64 , exists : bool , }
};
}
