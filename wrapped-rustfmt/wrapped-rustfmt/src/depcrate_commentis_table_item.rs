// Generated macro for is_table_item (function)
macro_rules! Depcrate_commentis_table_item {
() => {
// Module: crate::comment
// Provides: {"is_table_item"}
// Dependencies: {}
# [doc = " Returns true if the given string may be part of a Markdown table."] fn is_table_item (mut s : & str) -> bool { s = s . trim_start () ; return s . starts_with ('|') && match s . rfind ('|') { Some (0) | None => false , _ => true , } ; }
};
}
