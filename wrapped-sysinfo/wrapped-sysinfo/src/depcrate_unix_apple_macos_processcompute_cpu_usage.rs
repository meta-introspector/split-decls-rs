// Generated macro for compute_cpu_usage (function)
macro_rules! Depcrate_unix_apple_macos_processcompute_cpu_usage {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"compute_cpu_usage"}
// Dependencies: {}
# [allow (deprecated)] pub (crate) fn compute_cpu_usage (p : & mut ProcessInner , task_info : libc :: proc_taskinfo , system_time : u64 , user_time : u64 , time_interval : Option < f64 > ,) { if let Some (time_interval) = time_interval { let total_existing_time = p . old_stime . saturating_add (p . old_utime) ; if time_interval > 0.000001 && total_existing_time > 0 { let total_current_time = task_info . pti_total_system . saturating_add (task_info . pti_total_user) ; let total_time_diff = total_current_time . saturating_sub (total_existing_time) ; if total_time_diff > 0 { p . cpu_usage = (total_time_diff as f64 / time_interval * 100.) as f32 ; } } p . old_stime = task_info . pti_total_system ; p . old_utime = task_info . pti_total_user ; } else { unsafe { let time = libc :: mach_absolute_time () ; let task_time = user_time . saturating_add (system_time) . saturating_add (task_info . pti_total_user) . saturating_add (task_info . pti_total_system) ; let system_time_delta = if task_time < p . old_utime { task_time } else { task_time . saturating_sub (p . old_utime) } ; let time_delta = if time < p . old_stime { time } else { time . saturating_sub (p . old_stime) } ; p . old_utime = task_time ; p . old_stime = time ; p . cpu_usage = if time_delta == 0 { 0f32 } else { (system_time_delta as f64 * 100f64 / time_delta as f64) as f32 } ; } } }
};
}
