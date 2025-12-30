// Generated macro for load_avg (function)
macro_rules! Depcrate_windows_cpuload_avg {
() => {
// Module: crate::windows::cpu
// Provides: {"load_avg"}
// Dependencies: {}
fn load_avg () -> & 'static Mutex < Option < LoadAvg > > { static LOAD_AVG : OnceLock < Mutex < Option < LoadAvg > > > = OnceLock :: new () ; LOAD_AVG . get_or_init (| | unsafe { init_load_avg () }) }
};
}
