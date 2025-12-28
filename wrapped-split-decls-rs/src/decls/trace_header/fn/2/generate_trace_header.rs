use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Generates a trace header for any generated file"] pub fn generate_trace_header (operation : & str , input_file : Option < & str > , processor_function : & str , source_location : & str ,) -> String { generate_trace_header_with_comment_style (operation , input_file , processor_function , source_location , "//") }