// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_windows_cpuimpl_1080 {
() => {
// Module: crate::windows::cpu
// Provides: {"impl_1080"}
// Dependencies: {}
impl CpusWrapper { pub fn new () -> Self { Self { global : CpuUsage { percent : 0f32 , key_used : None , } , cpus : Vec :: new () , } } pub fn global_cpu_usage (& self) -> f32 { self . global . percent } pub fn cpus (& self) -> & [Cpu] { & self . cpus } fn init_if_needed (& mut self , refresh_kind : CpuRefreshKind) { if self . cpus . is_empty () { self . cpus = init_cpus (refresh_kind) ; } } pub fn len (& mut self) -> usize { self . init_if_needed (CpuRefreshKind :: nothing ()) ; self . cpus . len () } pub fn iter_mut (& mut self , refresh_kind : CpuRefreshKind) -> impl Iterator < Item = & mut Cpu > { self . init_if_needed (refresh_kind) ; self . cpus . iter_mut () } pub fn get_frequencies (& mut self) { let frequencies = get_frequencies (self . cpus . len ()) ; for (cpu , frequency) in self . cpus . iter_mut () . zip (frequencies) { cpu . inner . set_frequency (frequency) ; } } }
};
}
