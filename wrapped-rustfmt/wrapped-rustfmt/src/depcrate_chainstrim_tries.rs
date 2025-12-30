// Generated macro for trim_tries (function)
macro_rules! Depcrate_chainstrim_tries {
() => {
// Module: crate::chains
// Provides: {"trim_tries"}
// Dependencies: {}
# [doc = " Removes try operators (`?`s) that appear in the given string. If removing"] # [doc = " them leaves an empty line, remove that line as well unless it is the first"] # [doc = " line (we need the first newline for detecting pre/post comment)."] fn trim_tries (s : & str) -> String { let mut result = String :: with_capacity (s . len ()) ; let mut line_buffer = String :: with_capacity (s . len ()) ; for (kind , rich_char) in CharClasses :: new (s . chars ()) { match rich_char . get_char () { '\n' => { if result . is_empty () || ! line_buffer . trim () . is_empty () { result . push_str (& line_buffer) ; result . push ('\n') } line_buffer . clear () ; } '?' if kind == FullCodeCharKind :: Normal => continue , c => line_buffer . push (c) , } } if ! line_buffer . trim () . is_empty () { result . push_str (& line_buffer) ; } result }
};
}
