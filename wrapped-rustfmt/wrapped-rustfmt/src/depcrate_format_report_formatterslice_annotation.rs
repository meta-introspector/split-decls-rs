// Generated macro for slice_annotation (function)
macro_rules! Depcrate_format_report_formatterslice_annotation {
() => {
// Module: crate::format_report_formatter
// Provides: {"slice_annotation"}
// Dependencies: {}
fn slice_annotation (error : & FormattingError) -> Option < SourceAnnotation < '_ > > { let (range_start , range_length) = error . format_len () ; let range_end = range_start + range_length ; if range_length > 0 { Some (SourceAnnotation { annotation_type : AnnotationType :: Error , range : (range_start , range_end) , label : "" , }) } else { None } }
};
}
