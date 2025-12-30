// Generated macro for TestDesc (struct)
macro_rules! Depcrate_typesTestDesc {
() => {
// Module: crate::types
// Provides: {"TestDesc"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct TestDesc { pub name : TestName , pub ignore : bool , pub ignore_message : Option < & 'static str > , pub source_file : & 'static str , pub start_line : usize , pub start_col : usize , pub end_line : usize , pub end_col : usize , pub should_panic : options :: ShouldPanic , pub compile_fail : bool , pub no_run : bool , pub test_type : TestType , }
};
}
