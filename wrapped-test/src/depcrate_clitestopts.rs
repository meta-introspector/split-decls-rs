// Generated macro for TestOpts (struct)
macro_rules! Depcrate_cliTestOpts {
() => {
// Module: crate::cli
// Provides: {"TestOpts"}
// Dependencies: {}
# [derive (Debug)] pub struct TestOpts { pub list : bool , pub filters : Vec < String > , pub filter_exact : bool , pub force_run_in_process : bool , pub exclude_should_panic : bool , pub run_ignored : RunIgnored , pub run_tests : bool , pub bench_benchmarks : bool , pub logfile : Option < PathBuf > , pub nocapture : bool , pub color : ColorConfig , pub format : OutputFormat , pub shuffle : bool , pub shuffle_seed : Option < u64 > , pub test_threads : Option < usize > , pub skip : Vec < String > , pub time_options : Option < TestTimeOptions > , # [doc = " Stop at first failing test."] # [doc = " May run a few more tests due to threading, but will"] # [doc = " abort as soon as possible."] pub fail_fast : bool , pub options : Options , }
};
}
