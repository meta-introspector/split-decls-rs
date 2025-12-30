// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
pub mod test { pub use crate :: bench :: Bencher ; pub use crate :: cli :: { TestOpts , parse_opts } ; pub use crate :: helpers :: metrics :: { Metric , MetricMap } ; pub use crate :: options :: { Options , RunIgnored , RunStrategy , ShouldPanic } ; pub use crate :: test_result :: { TestResult , TrFailed , TrFailedMsg , TrIgnored , TrOk } ; pub use crate :: time :: { TestExecTime , TestTimeOptions } ; pub use crate :: types :: { DynTestFn , DynTestName , StaticBenchFn , StaticTestFn , StaticTestName , TestDesc , TestDescAndFn , TestId , TestName , TestType , } ; pub use crate :: { assert_test_result , filter_tests , run_test , test_main , test_main_static } ; }
};
}
