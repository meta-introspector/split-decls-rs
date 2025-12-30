// Generated macro for refresh_component (function)
macro_rules! Depcrate_unix_freebsd_componentrefresh_component {
() => {
// Module: crate::unix::freebsd::component
// Provides: {"refresh_component"}
// Dependencies: {}
unsafe fn refresh_component (id : & [u8]) -> Option < f32 > { let mut temperature : libc :: c_int = 0 ; if unsafe { ! get_sys_value_by_name (id , & mut temperature) } { None } else { Some ((temperature - 2732) as f32 / 10.) } }
};
}
