// Generated macro for Component (enum)
macro_rules! Depcrate_format_description_componentComponent {
() => {
// Module: crate::format_description::component
// Provides: {"Component"}
// Dependencies: {}
# [doc = " A component of a larger format description."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Component { # [doc = " Day of the month."] Day (modifier :: Day) , # [doc = " Month of the year."] Month (modifier :: Month) , # [doc = " Ordinal day of the year."] Ordinal (modifier :: Ordinal) , # [doc = " Day of the week."] Weekday (modifier :: Weekday) , # [doc = " Week within the year."] WeekNumber (modifier :: WeekNumber) , # [doc = " Year of the date."] Year (modifier :: Year) , # [doc = " Hour of the day."] Hour (modifier :: Hour) , # [doc = " Minute within the hour."] Minute (modifier :: Minute) , # [doc = " AM/PM part of the time."] Period (modifier :: Period) , # [doc = " Second within the minute."] Second (modifier :: Second) , # [doc = " Subsecond within the second."] Subsecond (modifier :: Subsecond) , # [doc = " Hour of the UTC offset."] OffsetHour (modifier :: OffsetHour) , # [doc = " Minute within the hour of the UTC offset."] OffsetMinute (modifier :: OffsetMinute) , # [doc = " Second within the minute of the UTC offset."] OffsetSecond (modifier :: OffsetSecond) , # [doc = " A number of bytes to ignore when parsing. This has no effect on formatting."] Ignore (modifier :: Ignore) , # [doc = " A Unix timestamp."] UnixTimestamp (modifier :: UnixTimestamp) , # [doc = " The end of input. Parsing this component will fail if there is any input remaining. This"] # [doc = " component neither affects formatting nor consumes any input when parsing."] End (modifier :: End) , }
};
}
