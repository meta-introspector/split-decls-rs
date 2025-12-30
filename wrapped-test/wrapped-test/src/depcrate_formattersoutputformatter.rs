// Generated macro for OutputFormatter (trait)
macro_rules! Depcrate_formattersOutputFormatter {
() => {
// Module: crate::formatters
// Provides: {"OutputFormatter"}
// Dependencies: {}
pub (crate) trait OutputFormatter { fn write_discovery_start (& mut self) -> io :: Result < () > ; fn write_test_discovered (& mut self , desc : & TestDesc , test_type : & str) -> io :: Result < () > ; fn write_discovery_finish (& mut self , state : & ConsoleTestDiscoveryState) -> io :: Result < () > ; fn write_run_start (& mut self , test_count : usize , shuffle_seed : Option < u64 >) -> io :: Result < () > ; fn write_test_start (& mut self , desc : & TestDesc) -> io :: Result < () > ; fn write_timeout (& mut self , desc : & TestDesc) -> io :: Result < () > ; fn write_result (& mut self , desc : & TestDesc , result : & TestResult , exec_time : Option < & time :: TestExecTime > , stdout : & [u8] , state : & ConsoleTestState ,) -> io :: Result < () > ; fn write_run_finish (& mut self , state : & ConsoleTestState) -> io :: Result < bool > ; fn write_merged_doctests_times (& mut self , total_time : f64 , compilation_time : f64 ,) -> io :: Result < () > ; }
};
}
