// Generated macro for impl_392 (impl)
macro_rules! Depcrate_cldr_serde_week_dataimpl_392 {
() => {
// Module: crate::cldr_serde::week_data
// Provides: {"impl_392"}
// Dependencies: {}
impl From < & Weekday > for icu :: calendar :: types :: Weekday { fn from (day : & Weekday) -> Self { use icu :: calendar :: types :: Weekday as CalWeekday ; match day { Weekday :: Mon => CalWeekday :: Monday , Weekday :: Tue => CalWeekday :: Tuesday , Weekday :: Wed => CalWeekday :: Wednesday , Weekday :: Thu => CalWeekday :: Thursday , Weekday :: Fri => CalWeekday :: Friday , Weekday :: Sat => CalWeekday :: Saturday , Weekday :: Sun => CalWeekday :: Sunday , } } }
};
}
