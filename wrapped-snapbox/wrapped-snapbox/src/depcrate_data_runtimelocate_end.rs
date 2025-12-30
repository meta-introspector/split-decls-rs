// Generated macro for locate_end (function)
macro_rules! Depcrate_data_runtimelocate_end {
() => {
// Module: crate::data::runtime
// Provides: {"locate_end"}
// Dependencies: {}
fn locate_end (arg_start_to_eof : & str) -> Option < usize > { match arg_start_to_eof . chars () . next () ? { c if c . is_whitespace () => panic ! ("skip whitespace before calling `locate_end`") , '[' => { let str_start_to_eof = arg_start_to_eof [1 ..] . trim_start () ; let str_len = find_str_lit_len (str_start_to_eof) ? ; let str_end_to_eof = & str_start_to_eof [str_len ..] ; let closing_brace_offset = str_end_to_eof . find (']') ? ; Some ((arg_start_to_eof . len () - str_end_to_eof . len ()) + closing_brace_offset + 1) } ']' | '}' | ')' => Some (0) , _ => find_str_lit_len (arg_start_to_eof) , } }
};
}
