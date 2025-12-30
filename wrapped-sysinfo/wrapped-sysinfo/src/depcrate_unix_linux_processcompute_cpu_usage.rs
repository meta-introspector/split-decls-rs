// Generated macro for compute_cpu_usage (function)
macro_rules! Depcrate_unix_linux_processcompute_cpu_usage {
() => {
// Module: crate::unix::linux::process
// Provides: {"compute_cpu_usage"}
// Dependencies: {}
pub (crate) fn compute_cpu_usage (p : & mut ProcessInner , total_time : f32 , max_value : f32) { if p . old_utime == 0 && p . old_stime == 0 { return ; } p . cpu_usage = (p . utime . saturating_sub (p . old_utime) . saturating_add (p . stime . saturating_sub (p . old_stime)) as f32 / total_time * 100.) . min (max_value) ; }
};
}
