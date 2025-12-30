// Generated macro for process_era_dates_map (function)
macro_rules! Depcrate_calendar_erasprocess_era_dates_map {
() => {
// Module: crate::calendar::eras
// Provides: {"process_era_dates_map"}
// Dependencies: {}
# [doc = " Aplies some fixes to the data"] fn process_era_dates_map (mut data : BTreeMap < String , cldr_serde :: eras :: CalendarData > ,) -> BTreeMap < String , cldr_serde :: eras :: CalendarData > { data . get_mut ("japanese") . unwrap () . eras = core :: mem :: take (& mut data . get_mut ("japanese") . unwrap () . eras) . into_iter () . map (| (idx , mut era) | { let idx = (idx . parse :: < usize > () . unwrap () + 2) . to_string () ; if let Some (start) = era . start . as_mut () { if start . month == 2 && start . day > 28 { start . day = if calendrical_calculations :: gregorian :: is_leap_year (start . year) { 29 } else { 28 } ; } } (idx , era) }) . collect () ; data }
};
}
