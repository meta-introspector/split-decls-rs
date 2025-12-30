// Generated macro for unalias_contexts (function)
macro_rules! Depcrate_datetime_legacyunalias_contexts {
() => {
// Module: crate::datetime::legacy
// Provides: {"unalias_contexts"}
// Dependencies: {}
# [test] fn unalias_contexts () { let provider = SourceDataProvider :: new_testing () ; let data = provider . get_dates_resource (& langid ! ("cs") . into () , Some (DatagenCalendar :: Gregorian)) . unwrap () ; let cs_dates = convert_dates (data , DatagenCalendar :: Gregorian) ; assert ! (cs_dates . months . stand_alone . is_some ()) ; assert ! (cs_dates . months . stand_alone . as_ref () . unwrap () . abbreviated . is_none ()) ; assert ! (cs_dates . months . stand_alone . as_ref () . unwrap () . short . is_none ()) ; assert ! (cs_dates . months . stand_alone . as_ref () . unwrap () . narrow . is_none ()) ; assert ! (cs_dates . months . stand_alone . as_ref () . unwrap () . wide . is_some ()) ; assert ! (! cs_dates . weekdays . stand_alone) ; }
};
}
