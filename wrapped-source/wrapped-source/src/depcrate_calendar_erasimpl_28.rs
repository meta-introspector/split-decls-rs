// Generated macro for impl_28 (impl)
macro_rules! Depcrate_calendar_erasimpl_28 {
() => {
// Module: crate::calendar::eras
// Provides: {"impl_28"}
// Dependencies: {}
impl SourceDataProvider { fn load_japanese_eras (& self , cal : DatagenCalendar ,) -> Result < DataResponse < CalendarJapaneseModernV1 > , DataError > { let mut dates_to_eras = BTreeMap :: new () ; for (_ , data) in self . all_eras () ? [& cal] . iter () . skip (2) { let start_date = data . start . unwrap () ; let code = data . code . as_deref () . unwrap () ; let code = code . parse () . map_err (| _ | { DataError :: custom ("Era code does not fit int TinyStr16") . with_display_context (& code) }) ? ; dates_to_eras . insert (start_date , code) ; } Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (JapaneseEras { dates_to_eras : dates_to_eras . into_iter () . collect () , }) , }) } }
};
}
