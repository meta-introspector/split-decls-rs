// Generated macro for heap_allocation_fails (function)
macro_rules! Depcrate_sys_pal_common_testsheap_allocation_fails {
() => {
// Module: crate::sys::pal::common::tests
// Provides: {"heap_allocation_fails"}
// Dependencies: {}
# [test] fn heap_allocation_fails () { let mut path = repeat ("a") . take (384) . collect :: < String > () ; path . push ('\0') ; let path = Path :: new (& path) ; assert ! (run_path_with_cstr ::< () > (path , &| _ | unreachable ! ()) . is_err ()) ; }
};
}
