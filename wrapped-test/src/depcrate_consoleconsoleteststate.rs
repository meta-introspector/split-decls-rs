// Generated macro for ConsoleTestState (struct)
macro_rules! Depcrate_consoleConsoleTestState {
() => {
// Module: crate::console
// Provides: {"ConsoleTestState"}
// Dependencies: {}
pub (crate) struct ConsoleTestState { pub log_out : Option < File > , pub total : usize , pub passed : usize , pub failed : usize , pub ignored : usize , pub filtered_out : usize , pub measured : usize , pub exec_time : Option < TestSuiteExecTime > , pub metrics : MetricMap , pub failures : Vec < (TestDesc , Vec < u8 >) > , pub not_failures : Vec < (TestDesc , Vec < u8 >) > , pub ignores : Vec < (TestDesc , Vec < u8 >) > , pub time_failures : Vec < (TestDesc , Vec < u8 >) > , pub options : Options , }
};
}
