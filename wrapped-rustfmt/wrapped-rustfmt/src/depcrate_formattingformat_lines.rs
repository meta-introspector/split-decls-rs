// Generated macro for format_lines (function)
macro_rules! Depcrate_formattingformat_lines {
() => {
// Module: crate::formatting
// Provides: {"format_lines"}
// Dependencies: {}
fn format_lines (text : & mut String , name : & FileName , skipped_range : & [(usize , usize)] , config : & Config , report : & FormatReport ,) { let mut formatter = FormatLines :: new (name , skipped_range , config) ; formatter . iterate (text) ; if formatter . newline_count > 1 { debug ! ("track truncate: {} {}" , text . len () , formatter . newline_count) ; let line = text . len () - formatter . newline_count + 1 ; text . truncate (line) ; } report . append (name . clone () , formatter . errors) ; }
};
}
