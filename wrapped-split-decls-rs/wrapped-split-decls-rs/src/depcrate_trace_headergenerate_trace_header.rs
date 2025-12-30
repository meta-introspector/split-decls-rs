// Generated macro for generate_trace_header (function)
macro_rules! Depcrate_trace_headergenerate_trace_header {
() => {
// Module: crate::trace_header
// Provides: {"generate_trace_header"}
// Dependencies: {}
# [doc = " Generates a trace header for any generated file"] pub fn generate_trace_header (operation : & str , input_file : Option < & str > , processor_function : & str , source_location : & str ,) -> String { generate_trace_header_with_comment_style (operation , input_file , processor_function , source_location , "//") }
};
}
