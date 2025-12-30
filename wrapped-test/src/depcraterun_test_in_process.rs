// Generated macro for run_test_in_process (function)
macro_rules! Depcraterun_test_in_process {
() => {
// Module: crate
// Provides: {"run_test_in_process"}
// Dependencies: {}
fn run_test_in_process (id : TestId , desc : TestDesc , nocapture : bool , report_time : bool , runnable_test : RunnableTest , monitor_ch : Sender < CompletedTest > , time_opts : Option < time :: TestTimeOptions > ,) { let data = Arc :: new (Mutex :: new (Vec :: new ())) ; if ! nocapture { io :: set_output_capture (Some (data . clone ())) ; } let start = report_time . then (Instant :: now) ; let result = fold_err (catch_unwind (AssertUnwindSafe (| | runnable_test . run ()))) ; let exec_time = start . map (| start | { let duration = start . elapsed () ; TestExecTime (duration) }) ; io :: set_output_capture (None) ; let test_result = calc_result (& desc , result . err () . as_deref () , time_opts . as_ref () , exec_time . as_ref ()) ; let stdout = data . lock () . unwrap_or_else (| e | e . into_inner ()) . to_vec () ; let message = CompletedTest :: new (id , desc , test_result , exec_time , stdout) ; monitor_ch . send (message) . unwrap () ; }
};
}
