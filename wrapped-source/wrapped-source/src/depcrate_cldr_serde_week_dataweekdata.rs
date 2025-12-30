// Generated macro for WeekData (struct)
macro_rules! Depcrate_cldr_serde_week_dataWeekData {
() => {
// Module: crate::cldr_serde::week_data
// Provides: {"WeekData"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename_all = "camelCase")] pub (crate) struct WeekData { pub (crate) min_days : BTreeMap < Territory , String > , pub (crate) first_day : BTreeMap < Territory , Weekday > , pub (crate) weekend_start : BTreeMap < Territory , Weekday > , pub (crate) weekend_end : BTreeMap < Territory , Weekday > , }
};
}
