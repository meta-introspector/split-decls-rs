// Generated macro for FormatLines (struct)
macro_rules! Depcrate_formattingFormatLines {
() => {
// Module: crate::formatting
// Provides: {"FormatLines"}
// Dependencies: {}
struct FormatLines < 'a > { name : & 'a FileName , skipped_range : & 'a [(usize , usize)] , last_was_space : bool , line_len : usize , cur_line : usize , newline_count : usize , errors : Vec < FormattingError > , line_buffer : String , current_line_contains_string_literal : bool , format_line : bool , config : & 'a Config , }
};
}
