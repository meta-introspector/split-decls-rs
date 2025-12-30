// Generated macro for days_in_feb (function)
macro_rules! Depcrate_timedays_in_feb {
() => {
// Module: crate::time
// Provides: {"days_in_feb"}
// Dependencies: {}
fn days_in_feb (year : u64) -> u64 { if (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0)) { 29 } else { 28 } }
};
}
