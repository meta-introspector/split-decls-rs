// Generated macro for heap_allocation_works (function)
macro_rules! Depcrate_sys_pal_common_testsheap_allocation_works {
() => {
// Module: crate::sys::pal::common::tests
// Provides: {"heap_allocation_works"}
// Dependencies: {}
# [test] fn heap_allocation_works () { let path = repeat ("a") . take (384) . collect :: < String > () ; let path = Path :: new (& path) ; let result = run_path_with_cstr (path , & | p | { assert_eq ! (p , &* CString :: new (path . as_os_str () . as_encoded_bytes ()) . unwrap ()) ; Ok (42) }) ; assert_eq ! (result . unwrap () , 42) ; }
};
}
