// Generated macro for get_result_from_exit_code (function)
macro_rules! Depcrate_test_resultget_result_from_exit_code {
() => {
// Module: crate::test_result
// Provides: {"get_result_from_exit_code"}
// Dependencies: {}
# [doc = " Creates a `TestResult` depending on the exit code of test subprocess."] pub (crate) fn get_result_from_exit_code (desc : & TestDesc , status : ExitStatus , time_opts : Option < & time :: TestTimeOptions > , exec_time : Option < & time :: TestExecTime > ,) -> TestResult { let result = match status . code () { Some (TR_OK) => TestResult :: TrOk , # [cfg (windows)] Some (STATUS_FAIL_FAST_EXCEPTION) => TestResult :: TrFailed , # [cfg (unix)] None => match status . signal () { Some (libc :: SIGABRT) => TestResult :: TrFailed , Some (signal) => { TestResult :: TrFailedMsg (format ! ("child process exited with signal {signal}")) } None => unreachable ! ("status.code() returned None but status.signal() was None") , } , # [cfg (target_os = "fuchsia")] Some (ZX_TASK_RETCODE_EXCEPTION_KILL) => TestResult :: TrFailed , # [cfg (not (unix))] None => TestResult :: TrFailedMsg (format ! ("unknown return code")) , # [cfg (any (windows , unix))] Some (code) => TestResult :: TrFailedMsg (format ! ("got unexpected return code {code}")) , # [cfg (not (any (windows , unix)))] Some (_) => TestResult :: TrFailed , } ; if result != TestResult :: TrOk { return result ; } if let (Some (opts) , Some (time)) = (time_opts , exec_time) { if opts . error_on_excess && opts . is_critical (desc , time) { return TestResult :: TrTimedFail ; } } result }
};
}
