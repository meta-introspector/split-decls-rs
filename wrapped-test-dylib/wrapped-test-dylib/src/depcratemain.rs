// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let ptr = unsafe { libc :: malloc (10) } ; unsafe { dep_free (ptr) } ; let ptr = unsafe { dep_malloc (10) } ; unsafe { libc :: free (ptr) } ; if cfg ! (feature = "override_allocator_on_supported_platforms") { let ptr = unsafe { tikv_jemalloc_sys :: malloc (10) } ; unsafe { dep_free (ptr) } ; let ptr = unsafe { tikv_jemalloc_sys :: malloc (10) } ; unsafe { libc :: free (ptr) } ; let ptr = unsafe { libc :: malloc (10) } ; unsafe { tikv_jemalloc_sys :: free (ptr) } ; let ptr = unsafe { dep_malloc (10) } ; unsafe { tikv_jemalloc_sys :: free (ptr) } ; } let dep = unsafe { CStr :: from_ptr (dep_lookup_malloc_address ()) } ; let here = unsafe { CStr :: from_ptr (lookup_malloc_address ()) } ; assert_eq ! (dep , here) ; }
};
}
