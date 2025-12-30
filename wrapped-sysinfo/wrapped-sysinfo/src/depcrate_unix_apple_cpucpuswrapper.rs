// Generated macro for CpusWrapper (struct)
macro_rules! Depcrate_unix_apple_cpuCpusWrapper {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"CpusWrapper"}
// Dependencies: {}
pub (crate) struct CpusWrapper { pub (crate) global_cpu : CpuUsage , pub (crate) cpus : Vec < Cpu > , pub (crate) got_cpu_frequency : bool , # [doc = " This field is needed to prevent updating when not enough time passed since last update."] last_update : Option < Instant > , }
};
}
