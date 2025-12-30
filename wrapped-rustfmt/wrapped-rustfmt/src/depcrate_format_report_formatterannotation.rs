// Generated macro for annotation (function)
macro_rules! Depcrate_format_report_formatterannotation {
() => {
// Module: crate::format_report_formatter
// Provides: {"annotation"}
// Dependencies: {}
fn annotation (error : & FormattingError) -> Option < Annotation < '_ > > { let (range_start , range_length) = error . format_len () ; let range_end = range_start + range_length ; if range_length > 0 { Some (Level :: Error . span (range_start .. range_end)) } else { None } }
};
}
