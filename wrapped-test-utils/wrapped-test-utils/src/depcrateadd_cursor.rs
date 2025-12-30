// Generated macro for add_cursor (function)
macro_rules! Depcrateadd_cursor {
() => {
// Module: crate
// Provides: {"add_cursor"}
// Dependencies: {}
# [doc = " Inserts `$0` marker into the `text` at `offset`."] pub fn add_cursor (text : & str , offset : TextSize) -> String { let offset : usize = offset . into () ; let mut res = String :: new () ; res . push_str (& text [.. offset]) ; res . push_str ("$0") ; res . push_str (& text [offset ..]) ; res }
};
}
