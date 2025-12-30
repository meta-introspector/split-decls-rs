// Generated macro for today (function)
macro_rules! Depcrate_timetoday {
() => {
// Module: crate::time
// Provides: {"today"}
// Dependencies: {}
pub fn today () -> Date { let default = Date { year : 2025 , month : 2 , day : 25 , } ; try_today () . unwrap_or (default) }
};
}
