use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn parse_file (file_path : & std :: path :: Path , threshold : usize) -> Result < (Vec < CodeBlockRef > , Tree , String) , String > { let source_code = fs :: read_to_string (file_path) . map_err (| _ | "Failed to read file") ? ; let mut parser = Parser :: new () ; let extension = file_path . extension () . and_then (| ext | ext . to_str ()) . ok_or ("Unsupported file extension") ? ; let language = get_language_from_extension (extension) . ok_or ("Unsupported file extension") ? ; set_parser_language (& mut parser , & language) ? ; let tree = parser . parse (& source_code , None) . ok_or ("Failed to parse code") ? ; let code_blocks = extract_code_blocks (tree . clone () , & source_code , threshold) ; Ok ((code_blocks , tree , source_code)) }
}