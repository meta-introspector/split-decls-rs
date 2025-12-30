// Generated macro for calendar_months (function)
macro_rules! Depcrate_datetime_neocalendar_months {
() => {
// Module: crate::datetime::neo
// Provides: {"calendar_months"}
// Dependencies: {}
# [doc = " Returns the number of regular months in a calendar, as well as whether it is"] # [doc = " has leap months"] fn calendar_months (cal : DatagenCalendar) -> (usize , bool) { match cal { DatagenCalendar :: Hebrew | DatagenCalendar :: Chinese | DatagenCalendar :: Dangi => (24 , true) , DatagenCalendar :: Coptic | DatagenCalendar :: Ethiopic => (13 , false) , DatagenCalendar :: Gregorian | DatagenCalendar :: Buddhist | DatagenCalendar :: JapaneseModern | DatagenCalendar :: JapaneseExtended | DatagenCalendar :: Indian | DatagenCalendar :: Persian | DatagenCalendar :: Hijri | DatagenCalendar :: Roc => (12 , false) , } }
};
}
