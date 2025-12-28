use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Finds all newlines, multi-byte characters, and non-narrow characters in a"] # [doc = " SourceFile."] # [doc = ""] # [doc = " This function will use an SSE2 enhanced implementation if hardware support"] # [doc = " is detected at runtime."] pub (crate) fn analyze_source_file (src : & str) -> (Vec < RelativeBytePos > , Vec < MultiByteChar >) { let mut lines = vec ! [RelativeBytePos :: from_u32 (0)] ; let mut multi_byte_chars = vec ! [] ; analyze_source_file_dispatch (src , & mut lines , & mut multi_byte_chars) ; if let Some (& last_line_start) = lines . last () { let source_file_end = RelativeBytePos :: from_usize (src . len ()) ; assert ! (source_file_end >= last_line_start) ; if last_line_start == source_file_end { lines . pop () ; } } (lines , multi_byte_chars) }
}