// Generated macro for ProcessInner (struct)
macro_rules! Depcrate_unix_linux_processProcessInner {
() => {
// Module: crate::unix::linux::process
// Provides: {"ProcessInner"}
// Dependencies: {}
pub (crate) struct ProcessInner { pub (crate) name : OsString , pub (crate) cmd : Vec < OsString > , pub (crate) exe : Option < PathBuf > , pub (crate) pid : Pid , parent : Option < Pid > , pub (crate) environ : Vec < OsString > , pub (crate) cwd : Option < PathBuf > , pub (crate) root : Option < PathBuf > , pub (crate) memory : u64 , pub (crate) virtual_memory : u64 , utime : u64 , stime : u64 , old_utime : u64 , old_stime : u64 , start_time_without_boot_time : u64 , start_time : u64 , start_time_raw : u64 , run_time : u64 , pub (crate) updated : bool , cpu_usage : f32 , user_id : Option < Uid > , effective_user_id : Option < Uid > , group_id : Option < Gid > , effective_group_id : Option < Gid > , pub (crate) status : ProcessStatus , pub (crate) tasks : Option < HashSet < Pid > > , stat_file : Option < FileCounter > , old_read_bytes : u64 , old_written_bytes : u64 , read_bytes : u64 , written_bytes : u64 , thread_kind : Option < ThreadKind > , proc_path : PathBuf , accumulated_cpu_time : u64 , exists : bool , }
};
}
