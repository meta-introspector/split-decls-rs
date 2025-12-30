// Generated macro for marker_filters (function)
macro_rules! Depcrate_relativetimemarker_filters {
() => {
// Module: crate::relativetime
// Provides: {"marker_filters"}
// Dependencies: {}
fn marker_filters () -> & 'static HashMap < DataMarkerInfo , & 'static str > { MARKER_FILTERS . get_or_init (| | { [(LongSecondRelativeV1 :: INFO , "second") , (ShortSecondRelativeV1 :: INFO , "second-short") , (NarrowSecondRelativeV1 :: INFO , "second-narrow") , (LongMinuteRelativeV1 :: INFO , "minute") , (ShortMinuteRelativeV1 :: INFO , "minute-short") , (NarrowMinuteRelativeV1 :: INFO , "minute-narrow") , (LongHourRelativeV1 :: INFO , "hour") , (ShortHourRelativeV1 :: INFO , "hour-short") , (NarrowHourRelativeV1 :: INFO , "hour-narrow") , (LongDayRelativeV1 :: INFO , "day") , (ShortDayRelativeV1 :: INFO , "day-short") , (NarrowDayRelativeV1 :: INFO , "day-narrow") , (LongWeekRelativeV1 :: INFO , "week") , (ShortWeekRelativeV1 :: INFO , "week-short") , (NarrowWeekRelativeV1 :: INFO , "week-narrow") , (LongMonthRelativeV1 :: INFO , "month") , (ShortMonthRelativeV1 :: INFO , "month-short") , (NarrowMonthRelativeV1 :: INFO , "month-narrow") , (LongQuarterRelativeV1 :: INFO , "quarter") , (ShortQuarterRelativeV1 :: INFO , "quarter-short") , (NarrowQuarterRelativeV1 :: INFO , "quarter-narrow") , (LongYearRelativeV1 :: INFO , "year") , (ShortYearRelativeV1 :: INFO , "year-short") , (NarrowYearRelativeV1 :: INFO , "year-narrow") ,] . into_iter () . collect () }) }
};
}
