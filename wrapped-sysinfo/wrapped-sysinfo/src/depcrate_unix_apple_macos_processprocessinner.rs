// Generated macro for ProcessInner (struct)
macro_rules! Depcrate_unix_apple_macos_processProcessInner {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"ProcessInner"}
// Dependencies: {}
pub (crate) struct ProcessInner { pub (crate) name : OsString , pub (crate) cmd : Vec < OsString > , pub (crate) exe : Option < PathBuf > , pid : Pid , parent : Option < Pid > , pub (crate) environ : Vec < OsString > , cwd : Option < PathBuf > , pub (crate) root : Option < PathBuf > , pub (crate) memory : u64 , pub (crate) virtual_memory : u64 , old_utime : u64 , old_stime : u64 , start_time : u64 , run_time : u64 , pub (crate) updated : bool , cpu_usage : f32 , user_id : Option < Uid > , effective_user_id : Option < Uid > , group_id : Option < Gid > , effective_group_id : Option < Gid > , pub (crate) process_status : ProcessStatus , # [doc = " Status of process (running, stopped, waiting, etc). `None` means `sysinfo` doesn't have"] # [doc = " enough rights to get this information."] # [doc = ""] # [doc = " This is very likely this one that you want instead of `process_status`."] pub (crate) status : Option < ThreadStatus > , pub (crate) old_read_bytes : u64 , pub (crate) old_written_bytes : u64 , pub (crate) read_bytes : u64 , pub (crate) written_bytes : u64 , accumulated_cpu_time : u64 , exists : bool , }
};
}
