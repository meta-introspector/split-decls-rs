// Generated macro for datetimepattern_convert (function)
macro_rules! Depcrate_datetime_neodatetimepattern_convert {
() => {
// Module: crate::datetime::neo
// Provides: {"datetimepattern_convert"}
// Dependencies: {}
fn datetimepattern_convert (data : & ca :: Dates , length : PatternLength , glue_type : GlueType ,) -> Result < GluePattern < 'static > , DataError > { let mut pattern_anchor = None ; let pattern = match glue_type { GlueType :: DateTime => data . datetime_formats_at_time . get_pattern (length) . get_pattern () , GlueType :: DateZone => { "{1} {2}" } GlueType :: TimeZone => { "{0} {2}" } GlueType :: DateTimeZone => { let pattern = pattern_anchor . insert (data . datetime_formats_at_time . get_pattern (length) . get_pattern () . to_string () ,) ; pattern . push_str (" {2}") ; pattern } } ; let pattern = pattern . parse () . expect ("failed to parse pattern") ; Ok (GluePattern { pattern }) }
};
}
