// Generated macro for run_tests_console (function)
macro_rules! Depcrate_consolerun_tests_console {
() => {
// Module: crate::console
// Provides: {"run_tests_console"}
// Dependencies: {}
# [doc = " A simple console test runner."] # [doc = " Runs provided tests reporting process and results to the stdout."] pub fn run_tests_console (opts : & TestOpts , tests : Vec < TestDescAndFn >) -> io :: Result < bool > { let max_name_len = tests . iter () . max_by_key (| t | len_if_padded (t)) . map (| t | t . desc . name . as_slice () . len ()) . unwrap_or (0) ; let mut out = get_formatter (opts , max_name_len) ; let mut st = ConsoleTestState :: new (opts) ? ; let is_instant_unsupported = (cfg ! (target_family = "wasm") && cfg ! (target_os = "unknown")) || cfg ! (target_os = "zkvm") ; let start_time = (! is_instant_unsupported) . then (Instant :: now) ; run_tests (opts , tests , | x | on_test_event (& x , & mut st , & mut * out)) ? ; st . exec_time = start_time . map (| t | TestSuiteExecTime (t . elapsed ())) ; assert ! (opts . fail_fast || st . current_test_count () == st . total) ; out . write_run_finish (& st) }
};
}
