// Generated macro for test_child_doesnt_ref_parent (function)
macro_rules! Depcrate_thread_teststest_child_doesnt_ref_parent {
() => {
// Module: crate::thread::tests
// Provides: {"test_child_doesnt_ref_parent"}
// Dependencies: {}
# [test] fn test_child_doesnt_ref_parent () { const GENERATIONS : u32 = 16 ; fn child_no (x : u32) -> Box < dyn Fn () + Send > { return Box :: new (move | | { if x < GENERATIONS { thread :: spawn (move | | child_no (x + 1) ()) ; } }) ; } thread :: spawn (| | child_no (0) ()) ; }
};
}
