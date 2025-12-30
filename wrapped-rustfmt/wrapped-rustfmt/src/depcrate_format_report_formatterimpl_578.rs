// Generated macro for impl_578 (impl)
macro_rules! Depcrate_format_report_formatterimpl_578 {
() => {
// Module: crate::format_report_formatter
// Provides: {"impl_578"}
// Dependencies: {}
impl < 'a > Display for FormatReportFormatter < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let errors_by_file = & self . report . internal . borrow () . 0 ; let renderer = if self . enable_colors { Renderer :: styled () } else { Renderer :: plain () } ; for (file , errors) in errors_by_file { for error in errors { let error_kind = error . kind . to_string () ; let mut message = error_kind_to_snippet_annotation_level (& error . kind) . title (& error_kind) ; if error . is_internal () { message = message . id ("internal") ; } let message_suffix = error . msg_suffix () ; if ! message_suffix . is_empty () { message = message . footer (Level :: Note . title (& message_suffix)) ; } let origin = format ! ("{}:{}" , file , error . line) ; let snippet = Snippet :: source (& error . line_buffer) . line_start (error . line) . origin (& origin) . fold (false) . annotations (annotation (error)) ; message = message . snippet (snippet) ; writeln ! (f , "{}\n" , renderer . render (message)) ? ; } } if ! errors_by_file . is_empty () { let label = format ! ("rustfmt has failed to format. See previous {} errors." , self . report . warning_count ()) ; let message = Level :: Warning . title (& label) ; writeln ! (f , "{}" , renderer . render (message)) ? ; } Ok (()) } }
};
}
