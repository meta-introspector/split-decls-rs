// Generated macro for impl_29 (impl)
macro_rules! Depcrate_calendar_erasimpl_29 {
() => {
// Module: crate::calendar::eras
// Provides: {"impl_29"}
// Dependencies: {}
impl DataProvider < CalendarJapaneseModernV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CalendarJapaneseModernV1 > , DataError > { self . check_req :: < CalendarJapaneseModernV1 > (req) ? ; self . load_japanese_eras (DatagenCalendar :: JapaneseModern) } }
};
}
