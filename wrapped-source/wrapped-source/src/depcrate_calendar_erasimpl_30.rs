// Generated macro for impl_30 (impl)
macro_rules! Depcrate_calendar_erasimpl_30 {
() => {
// Module: crate::calendar::eras
// Provides: {"impl_30"}
// Dependencies: {}
impl DataProvider < CalendarJapaneseExtendedV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < CalendarJapaneseExtendedV1 > , DataError > { self . check_req :: < CalendarJapaneseExtendedV1 > (req) ? ; let DataResponse { metadata , payload } = self . load_japanese_eras (DatagenCalendar :: JapaneseExtended) ? ; if env :: var ("ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK") . is_err () { let snapshot : JapaneseEras = serde_json :: from_str (JAPANEXT_FILE) . expect ("Failed to parse the precached golden. This is a bug.") ; if snapshot != * payload . get () { return Err (DataError :: custom ("Era data has changed! This can be for two reasons: Either the CLDR locale data for Japanese eras has \
                    changed in an incompatible way, or there is a new Japanese era. Run \
                    `ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK=1 cargo run -p icu4x-datagen -- --markers CalendarJapaneseExtendedV1 --format fs --syntax json \
                    --out provider/source/data/japanese-golden --pretty --overwrite` in the icu4x repo and inspect the diff to \
                    check which situation it is. If a new era has been introduced, commit the diff, if not, it's likely that japanese.rs \
                    in icu_provider_source will need to be updated to handle the data changes.")) ; } } Ok (DataResponse { metadata , payload : payload . cast () , }) } }
};
}
