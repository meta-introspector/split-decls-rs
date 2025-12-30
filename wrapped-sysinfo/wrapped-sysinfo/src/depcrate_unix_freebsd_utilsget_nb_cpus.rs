// Generated macro for get_nb_cpus (function)
macro_rules! Depcrate_unix_freebsd_utilsget_nb_cpus {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_nb_cpus"}
// Dependencies: {}
# [cfg (any (feature = "system" , feature = "component"))] pub (crate) unsafe fn get_nb_cpus () -> usize { let mut smp : libc :: c_int = 0 ; let mut nb_cpus : libc :: c_int = 1 ; unsafe { if ! get_sys_value_by_name (b"kern.smp.active\0" , & mut smp) { smp = 0 ; } # [allow (clippy :: collapsible_if)] if smp != 0 { if ! get_sys_value_by_name (b"kern.smp.cpus\0" , & mut nb_cpus) || nb_cpus < 1 { nb_cpus = 1 ; } } } nb_cpus as usize }
};
}
