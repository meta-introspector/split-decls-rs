// Generated macro for list_tests_console (function)
macro_rules! Depcrate_consolelist_tests_console {
() => {
// Module: crate::console
// Provides: {"list_tests_console"}
// Dependencies: {}
pub (crate) fn list_tests_console (opts : & TestOpts , tests : Vec < TestDescAndFn >) -> io :: Result < () > { let output = match term :: stdout () { None => OutputLocation :: Raw (io :: stdout () . lock ()) , Some (t) => OutputLocation :: Pretty (t) , } ; let mut out : Box < dyn OutputFormatter > = match opts . format { OutputFormat :: Pretty | OutputFormat :: Junit => { Box :: new (PrettyFormatter :: new (output , false , 0 , false , None)) } OutputFormat :: Terse => Box :: new (TerseFormatter :: new (output , false , 0 , false)) , OutputFormat :: Json => Box :: new (JsonFormatter :: new (output)) , } ; let mut st = ConsoleTestDiscoveryState :: new (opts) ? ; out . write_discovery_start () ? ; for test in filter_tests (opts , tests) . into_iter () { use crate :: TestFn :: * ; let TestDescAndFn { desc , testfn } = test ; let fntype = match testfn { StaticTestFn (..) | DynTestFn (..) | StaticBenchAsTestFn (..) | DynBenchAsTestFn (..) => { st . tests += 1 ; "test" } StaticBenchFn (..) | DynBenchFn (..) => { st . benchmarks += 1 ; "benchmark" } } ; st . ignored += if desc . ignore { 1 } else { 0 } ; out . write_test_discovered (& desc , fntype) ? ; st . write_log (| | format ! ("{fntype} {}\n" , desc . name)) ? ; } out . write_discovery_finish (& st) }
};
}
