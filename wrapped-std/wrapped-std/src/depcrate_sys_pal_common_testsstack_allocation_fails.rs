// Generated macro for stack_allocation_fails (function)
macro_rules! Depcrate_sys_pal_common_testsstack_allocation_fails {
() => {
// Module: crate::sys::pal::common::tests
// Provides: {"stack_allocation_fails"}
// Dependencies: {}
# [test] fn stack_allocation_fails () { let path = Path :: new ("ab\0") ; assert ! (run_path_with_cstr ::< () > (path , &| _ | unreachable ! ()) . is_err ()) ; }
};
}
