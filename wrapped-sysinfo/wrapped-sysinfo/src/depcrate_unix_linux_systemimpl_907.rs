// Generated macro for impl_907 (impl)
macro_rules! Depcrate_unix_linux_systemimpl_907 {
() => {
// Module: crate::unix::linux::system
// Provides: {"impl_907"}
// Dependencies: {}
impl SystemInner { # [doc = " It is sometime possible that a CPU usage computation is bigger than"] # [doc = " `\"number of CPUs\" * 100`."] # [doc = ""] # [doc = " To prevent that, we compute ahead of time this maximum value and ensure that processes'"] # [doc = " CPU usage don't go over it."] fn get_max_process_cpu_usage (& self) -> f32 { self . cpus . len () as f32 * 100. } fn update_procs_cpu (& mut self , refresh_kind : ProcessRefreshKind) { if ! refresh_kind . cpu () { return ; } self . cpus . refresh_if_needed (true , CpuRefreshKind :: nothing () . with_cpu_usage ()) ; if self . cpus . is_empty () { sysinfo_debug ! ("cannot compute processes CPU usage: no CPU found...") ; return ; } let (new , old) = self . cpus . get_global_raw_times () ; let total_time = if old > new { 1 } else { new - old } ; let total_time = total_time as f32 / self . cpus . len () as f32 ; let max_value = self . get_max_process_cpu_usage () ; for proc_ in self . process_list . values_mut () { compute_cpu_usage (& mut proc_ . inner , total_time , max_value) ; } } fn refresh_cpus (& mut self , only_update_global_cpu : bool , refresh_kind : CpuRefreshKind) { self . cpus . refresh (only_update_global_cpu , refresh_kind) ; } }
};
}
