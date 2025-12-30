// Generated macro for impl_330 (impl)
macro_rules! Depcrate_unix_apple_cpuimpl_330 {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"impl_330"}
// Dependencies: {}
impl CpusWrapper { pub (crate) fn new () -> Self { Self { global_cpu : CpuUsage :: new () , cpus : Vec :: new () , got_cpu_frequency : false , last_update : None , } } pub (crate) fn refresh (& mut self , refresh_kind : CpuRefreshKind , port : mach_port_t) { let need_cpu_usage_update = self . last_update . is_some_and (| last_update | last_update . elapsed () >= crate :: MINIMUM_CPU_UPDATE_INTERVAL) ; let cpus = & mut self . cpus ; if cpus . is_empty () { init_cpus (port , cpus , & mut self . global_cpu , refresh_kind) ; self . last_update = Some (Instant :: now ()) ; self . got_cpu_frequency = refresh_kind . frequency () ; return ; } if refresh_kind . frequency () && ! self . got_cpu_frequency { let frequency = unsafe { get_cpu_frequency (cpus . first () . map_or ("" , | c | c . brand ())) } ; for proc_ in cpus . iter_mut () { proc_ . inner . set_frequency (frequency) ; } self . got_cpu_frequency = true ; } if refresh_kind . cpu_usage () && need_cpu_usage_update { self . last_update = Some (Instant :: now ()) ; update_cpu_usage (port , & mut self . global_cpu , | proc_data , cpu_info | { let mut percentage = 0f32 ; let mut offset = 0 ; for proc_ in cpus . iter_mut () { let cpu_usage = compute_usage_of_cpu (proc_ , cpu_info , offset) ; proc_ . inner . update (cpu_usage , Arc :: clone (& proc_data)) ; percentage += proc_ . inner . cpu_usage () ; offset += libc :: CPU_STATE_MAX as isize ; } (percentage , cpus . len ()) }) ; } } }
};
}
