// Generated macro for days_before_year_since_unix_epoch (function)
macro_rules! Depcrate_timedays_before_year_since_unix_epoch {
() => {
// Module: crate::time
// Provides: {"days_before_year_since_unix_epoch"}
// Dependencies: {}
fn days_before_year_since_unix_epoch (year : u64) -> Result < u64 , Error > { if year < UNIX_EPOCH_YEAR { return Err (Error :: BadDerTime) ; } let days_before_year_ad = days_before_year_ad (year) ; debug_assert ! (days_before_year_ad >= DAYS_BEFORE_UNIX_EPOCH_AD) ; Ok (days_before_year_ad - DAYS_BEFORE_UNIX_EPOCH_AD) }
};
}
