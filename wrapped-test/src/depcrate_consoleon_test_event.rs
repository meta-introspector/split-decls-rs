// Generated macro for on_test_event (function)
macro_rules! Depcrate_consoleon_test_event {
() => {
// Module: crate::console
// Provides: {"on_test_event"}
// Dependencies: {}
fn on_test_event (event : & TestEvent , st : & mut ConsoleTestState , out : & mut dyn OutputFormatter ,) -> io :: Result < () > { match (* event) . clone () { TestEvent :: TeFiltered (filtered_tests , shuffle_seed) => { st . total = filtered_tests ; out . write_run_start (filtered_tests , shuffle_seed) ? ; } TestEvent :: TeFilteredOut (filtered_out) => { st . filtered_out = filtered_out ; } TestEvent :: TeWait (ref test) => out . write_test_start (test) ? , TestEvent :: TeTimeout (ref test) => out . write_timeout (test) ? , TestEvent :: TeResult (completed_test) => { let test = & completed_test . desc ; let result = & completed_test . result ; let exec_time = & completed_test . exec_time ; let stdout = & completed_test . stdout ; st . write_log_result (test , result , exec_time . as_ref ()) ? ; out . write_result (test , result , exec_time . as_ref () , stdout , st) ? ; handle_test_result (st , completed_test) ; } } Ok (()) }
};
}
