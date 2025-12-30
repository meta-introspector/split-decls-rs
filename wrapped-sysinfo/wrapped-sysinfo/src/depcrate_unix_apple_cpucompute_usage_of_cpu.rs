// Generated macro for compute_usage_of_cpu (function)
macro_rules! Depcrate_unix_apple_cpucompute_usage_of_cpu {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"compute_usage_of_cpu"}
// Dependencies: {}
pub (crate) fn compute_usage_of_cpu (proc_ : & Cpu , cpu_info : * mut i32 , offset : isize) -> f32 { let old_cpu_info = proc_ . inner . data () . cpu_info . 0 ; let in_use ; let idle ; if std :: ptr :: eq (old_cpu_info , cpu_info) { in_use = get_in_use (cpu_info , offset) ; idle = get_idle (cpu_info , offset) ; } else { let new_in_use = get_in_use (cpu_info , offset) ; let old_in_use = get_in_use (old_cpu_info , offset) ; let new_idle = get_idle (cpu_info , offset) ; let old_idle = get_idle (old_cpu_info , offset) ; in_use = new_in_use . saturating_sub (old_in_use) ; idle = new_idle . saturating_sub (old_idle) as _ ; } let total = in_use . saturating_add (idle as _) ; let usage = (in_use as f32 / total as f32) * 100. ; if usage . is_nan () { 0. } else { usage } }
};
}
