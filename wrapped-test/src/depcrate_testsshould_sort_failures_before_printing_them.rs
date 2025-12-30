// Generated macro for should_sort_failures_before_printing_them (function)
macro_rules! Depcrate_testsshould_sort_failures_before_printing_them {
() => {
// Module: crate::tests
// Provides: {"should_sort_failures_before_printing_them"}
// Dependencies: {}
# [test] fn should_sort_failures_before_printing_them () { let test_a = TestDesc { name : StaticTestName ("a") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } ; let test_b = TestDesc { name : StaticTestName ("b") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } ; let mut out = PrettyFormatter :: new (OutputLocation :: Raw (Vec :: new ()) , false , 10 , false , None) ; let st = console :: ConsoleTestState { log_out : None , total : 0 , passed : 0 , failed : 0 , ignored : 0 , filtered_out : 0 , measured : 0 , exec_time : None , metrics : MetricMap :: new () , failures : vec ! [(test_b , Vec :: new ()) , (test_a , Vec :: new ())] , options : Options :: new () , not_failures : Vec :: new () , ignores : Vec :: new () , time_failures : Vec :: new () , } ; out . write_failures (& st) . unwrap () ; let s = match out . output_location () { & OutputLocation :: Raw (ref m) => String :: from_utf8_lossy (& m [..]) , & OutputLocation :: Pretty (_) => unreachable ! () , } ; let apos = s . find ("a") . unwrap () ; let bpos = s . find ("b") . unwrap () ; assert ! (apos < bpos) ; }
};
}
