// Generated macro for run_test_in_spawned_subprocess (function)
macro_rules! Depcraterun_test_in_spawned_subprocess {
() => {
// Module: crate
// Provides: {"run_test_in_spawned_subprocess"}
// Dependencies: {}
fn run_test_in_spawned_subprocess (desc : TestDesc , runnable_test : RunnableTest) -> ! { let builtin_panic_hook = panic :: take_hook () ; let record_result = Arc :: new (move | panic_info : Option < & '_ PanicHookInfo < '_ > > | { let test_result = calc_result (& desc , panic_info . map (| info | info . payload ()) , None , None) ; if let TrFailedMsg (msg) = & test_result { eprintln ! ("{msg}") ; } if let Some (info) = panic_info { builtin_panic_hook (info) ; } if let TrOk = test_result { process :: exit (test_result :: TR_OK) ; } else { process :: abort () ; } }) ; let record_result2 = record_result . clone () ; panic :: set_hook (Box :: new (move | info | record_result2 (Some (info)))) ; if let Err (message) = runnable_test . run () { panic ! ("{}" , message) ; } record_result (None) ; unreachable ! ("panic=abort callback should have exited the process") }
};
}
