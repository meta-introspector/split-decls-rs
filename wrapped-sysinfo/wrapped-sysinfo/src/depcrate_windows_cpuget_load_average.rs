// Generated macro for get_load_average (function)
macro_rules! Depcrate_windows_cpuget_load_average {
() => {
// Module: crate::windows::cpu
// Provides: {"get_load_average"}
// Dependencies: {}
pub (crate) fn get_load_average () -> LoadAvg { if let Ok (avg) = load_avg () . lock () && let Some (avg) = & * avg { return avg . clone () ; } LoadAvg :: default () }
};
}
