// Generated macro for calc_result (function)
macro_rules! Depcrate_test_resultcalc_result {
() => {
// Module: crate::test_result
// Provides: {"calc_result"}
// Dependencies: {}
# [doc = " Creates a `TestResult` depending on the raw result of test execution"] # [doc = " and associated data."] pub (crate) fn calc_result (desc : & TestDesc , panic_payload : Option < & (dyn Any + Send) > , time_opts : Option < & time :: TestTimeOptions > , exec_time : Option < & time :: TestExecTime > ,) -> TestResult { let result = match (desc . should_panic , panic_payload) { (ShouldPanic :: No , None) | (ShouldPanic :: Yes , Some (_)) => TestResult :: TrOk , (ShouldPanic :: YesWithMessage (msg) , Some (err)) => { let maybe_panic_str = err . downcast_ref :: < String > () . map (| e | & * * e) . or_else (| | err . downcast_ref :: < & 'static str > () . copied ()) ; if maybe_panic_str . map (| e | e . contains (msg)) . unwrap_or (false) { TestResult :: TrOk } else if let Some (panic_str) = maybe_panic_str { TestResult :: TrFailedMsg (format ! (r#"panic did not contain expected string
      panic message: {panic_str:?}
 expected substring: {msg:?}"#)) } else { TestResult :: TrFailedMsg (format ! (r#"expected panic with string value,
 found non-string value: `{:?}`
     expected substring: {msg:?}"# , (* err) . type_id ())) } } (ShouldPanic :: Yes , None) | (ShouldPanic :: YesWithMessage (_) , None) => { let fn_location = if ! desc . source_file . is_empty () { & format ! (" at {}:{}:{}" , desc . source_file , desc . start_line , desc . start_col) } else { "" } ; TestResult :: TrFailedMsg (format ! ("test did not panic as expected{}" , fn_location)) } (ShouldPanic :: No , Some (_)) => TestResult :: TrFailed , } ; if result != TestResult :: TrOk { return result ; } if let (Some (opts) , Some (time)) = (time_opts , exec_time) { if opts . error_on_excess && opts . is_critical (desc , time) { return TestResult :: TrTimedFail ; } } result }
};
}
