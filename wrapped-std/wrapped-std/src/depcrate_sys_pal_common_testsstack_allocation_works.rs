// Generated macro for stack_allocation_works (function)
macro_rules! Depcrate_sys_pal_common_testsstack_allocation_works {
() => {
// Module: crate::sys::pal::common::tests
// Provides: {"stack_allocation_works"}
// Dependencies: {}
# [test] fn stack_allocation_works () { let path = Path :: new ("abc") ; let result = run_path_with_cstr (path , & | p | { assert_eq ! (p , &* CString :: new (path . as_os_str () . as_encoded_bytes ()) . unwrap ()) ; Ok (42) }) ; assert_eq ! (result . unwrap () , 42) ; }
};
}
