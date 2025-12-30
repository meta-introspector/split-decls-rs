// Generated macro for days_before_year_ad (function)
macro_rules! Depcrate_timedays_before_year_ad {
() => {
// Module: crate::time
// Provides: {"days_before_year_ad"}
// Dependencies: {}
fn days_before_year_ad (year : u64) -> u64 { ((year - 1) * 365) + ((year - 1) / 4) - ((year - 1) / 100) + ((year - 1) / 400) }
};
}
