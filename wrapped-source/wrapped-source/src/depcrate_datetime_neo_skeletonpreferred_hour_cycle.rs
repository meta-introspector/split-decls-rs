// Generated macro for preferred_hour_cycle (function)
macro_rules! Depcrate_datetime_neo_skeletonpreferred_hour_cycle {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"preferred_hour_cycle"}
// Dependencies: {}
fn preferred_hour_cycle (other : & cldr_serde :: ca :: Dates , locale : & DataLocale) -> CoarseHourCycle { let mut preferred_hour_cycle : Option < CoarseHourCycle > = None ; for s in [& other . time_skeletons . full , & other . time_skeletons . long , & other . time_skeletons . medium , & other . time_skeletons . short ,] { let Some (hour_cycle) = pattern :: CoarseHourCycle :: determine (& s . get_pattern () . parse () . expect ("Failed to crate pattern from bytes") ,) else { continue ; } ; if let Some (preferred_hour_cycle) = preferred_hour_cycle { if hour_cycle != preferred_hour_cycle { log :: warn ! ("{locale:?} contained a mix of coarse hour cycle types ({hour_cycle:?}, {preferred_hour_cycle:?})") ; } } else { preferred_hour_cycle = Some (hour_cycle) ; } } preferred_hour_cycle . expect ("Could not find a preferred hour cycle.") }
};
}
