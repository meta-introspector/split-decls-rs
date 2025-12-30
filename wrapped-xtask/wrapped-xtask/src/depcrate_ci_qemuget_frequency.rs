// Generated macro for get_frequency (function)
macro_rules! Depcrate_ci_qemuget_frequency {
() => {
// Module: crate::ci::qemu
// Provides: {"get_frequency"}
// Dependencies: {}
fn get_frequency () -> u64 { let mut sys = System :: new () ; sys . refresh_cpu_specifics (CpuRefreshKind :: nothing () . with_frequency ()) ; let frequency = sys . cpus () . first () . unwrap () . frequency () ; if ! sys . cpus () . iter () . all (| cpu | cpu . frequency () == frequency) { eprintln ! ("CPU frequencies are not all equal") ; } frequency }
};
}
