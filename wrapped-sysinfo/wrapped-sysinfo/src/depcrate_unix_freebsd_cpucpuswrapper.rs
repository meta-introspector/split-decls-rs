// Generated macro for CpusWrapper (struct)
macro_rules! Depcrate_unix_freebsd_cpuCpusWrapper {
() => {
// Module: crate::unix::freebsd::cpu
// Provides: {"CpusWrapper"}
// Dependencies: {}
pub (crate) struct CpusWrapper { pub (crate) global_cpu_usage : f32 , pub (crate) cpus : Vec < Cpu > , got_cpu_frequency : bool , mib_cp_time : [c_int ; 2] , mib_cp_times : [c_int ; 2] , cp_time : VecSwitcher < c_ulong > , cp_times : VecSwitcher < c_ulong > , nb_cpus : usize , }
};
}
