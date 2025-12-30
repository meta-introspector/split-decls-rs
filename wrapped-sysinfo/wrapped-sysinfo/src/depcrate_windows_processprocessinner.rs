// Generated macro for ProcessInner (struct)
macro_rules! Depcrate_windows_processProcessInner {
() => {
// Module: crate::windows::process
// Provides: {"ProcessInner"}
// Dependencies: {}
pub (crate) struct ProcessInner { name : OsString , cmd : Vec < OsString > , exe : Option < PathBuf > , pid : Pid , user_id : Option < Uid > , environ : Vec < OsString > , cwd : Option < PathBuf > , root : Option < PathBuf > , pub (crate) memory : u64 , pub (crate) virtual_memory : u64 , pub (crate) parent : Option < Pid > , status : ProcessStatus , handle : Option < Arc < HandleWrapper > > , cpu_calc_values : CPUsageCalculationValues , start_time : u64 , pub (crate) run_time : u64 , cpu_usage : f32 , pub (crate) updated : bool , old_read_bytes : u64 , old_written_bytes : u64 , read_bytes : u64 , written_bytes : u64 , accumulated_cpu_time : u64 , exists : bool , }
};
}
