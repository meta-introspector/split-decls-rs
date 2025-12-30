// Generated macro for impl_318 (impl)
macro_rules! Depcrate_typesimpl_318 {
() => {
// Module: crate::types
// Provides: {"impl_318"}
// Dependencies: {}
impl TestDescAndFn { pub const fn new_doctest (test_name : & 'static str , ignore : bool , source_file : & 'static str , start_line : usize , no_run : bool , should_panic : bool , testfn : TestFn ,) -> Self { Self { desc : TestDesc { name : StaticTestName (test_name) , ignore , ignore_message : None , source_file , start_line , start_col : 0 , end_line : 0 , end_col : 0 , compile_fail : false , no_run , should_panic : if should_panic { options :: ShouldPanic :: Yes } else { options :: ShouldPanic :: No } , test_type : TestType :: DocTest , } , testfn , } } }
};
}
