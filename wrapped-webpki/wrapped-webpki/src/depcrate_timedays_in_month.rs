// Generated macro for days_in_month (function)
macro_rules! Depcrate_timedays_in_month {
() => {
// Module: crate::time
// Provides: {"days_in_month"}
// Dependencies: {}
pub (crate) fn days_in_month (year : u64 , month : u64) -> u64 { match month { 1 | 3 | 5 | 7 | 8 | 10 | 12 => 31 , 4 | 6 | 9 | 11 => 30 , 2 => days_in_feb (year) , _ => unreachable ! () , } }
};
}
