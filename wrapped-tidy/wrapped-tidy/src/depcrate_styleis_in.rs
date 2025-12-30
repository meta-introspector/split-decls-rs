// Generated macro for is_in (function)
macro_rules! Depcrate_styleis_in {
() => {
// Module: crate::style
// Provides: {"is_in"}
// Dependencies: {}
pub fn is_in (full_path : & Path , parent_folder_to_find : & str , folder_to_find : & str) -> bool { if let Some (parent) = full_path . parent () { if parent . file_name () . map_or_else (| | false , | f | { f == folder_to_find && parent . parent () . and_then (| f | f . file_name ()) . map_or_else (| | false , | f | f == parent_folder_to_find) } ,) { true } else { is_in (parent , parent_folder_to_find , folder_to_find) } } else { false } }
};
}
