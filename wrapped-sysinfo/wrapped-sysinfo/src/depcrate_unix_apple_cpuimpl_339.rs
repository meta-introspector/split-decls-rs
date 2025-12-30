// Generated macro for impl_339 (impl)
macro_rules! Depcrate_unix_apple_cpuimpl_339 {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"impl_339"}
// Dependencies: {}
impl CpuUsage { pub (crate) fn new () -> Self { Self { percent : 0. , data : Arc :: new (CpuData :: new (std :: ptr :: null_mut () , 0)) , frequency : 0 , } } pub (crate) fn percent (& self) -> f32 { self . percent } pub (crate) fn set_cpu_usage (& mut self , value : f32) { self . percent = value ; } }
};
}
