// Generated macro for finish_all (function)
macro_rules! Depcrate_uifinish_all {
() => {
// Module: crate::ui
// Provides: {"finish_all"}
// Dependencies: {}
# [doc = " Print final messages after all tests are complete."] pub fn finish_all (tests : & [TestInfo] , total_elapsed : Duration , cfg : & Config) -> ExitCode { println ! ("\n\nResults:") ; let mut failed_generators = 0 ; let mut stopped_generators = 0 ; for t in tests { let Completed { executed , failures , elapsed , warning , result } = t . completed . get () . unwrap () ; let stat = if result . is_err () { stopped_generators += 1 ; "STOPPED" } else if * failures > 0 { failed_generators += 1 ; "FAILURE" } else { "SUCCESS" } ; println ! ("    {stat} for generator '{name}'. {passed}/{executed} passed in {elapsed:?}" , name = t . name , passed = executed - failures ,) ; if let Some (warning) = warning { println ! ("      warning: {warning}") ; } match result { Ok (FinishedAll) => () , Err (EarlyExit :: Timeout) => { println ! ("      exited early; exceeded {:?} timeout" , cfg . timeout) } Err (EarlyExit :: MaxFailures) => { println ! ("      exited early; exceeded {:?} max failures" , cfg . max_failures) } } } println ! ("{passed}/{} tests succeeded in {total_elapsed:?} ({passed} passed, {} failed, {} stopped)" , tests . len () , failed_generators , stopped_generators , passed = tests . len () - failed_generators - stopped_generators ,) ; if failed_generators > 0 || stopped_generators > 0 { ExitCode :: FAILURE } else { ExitCode :: SUCCESS } }
};
}
