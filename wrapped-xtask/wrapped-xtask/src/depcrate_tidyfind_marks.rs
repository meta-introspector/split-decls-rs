// Generated macro for find_marks (function)
macro_rules! Depcrate_tidyfind_marks {
() => {
// Module: crate::tidy
// Provides: {"find_marks"}
// Dependencies: {}
fn find_marks (set : & mut HashSet < String > , text : & str , mark : & str) { let mut text = text ; let mut prev_text = "" ; while text != prev_text { prev_text = text ; if let Some (idx) = text . find (mark) { text = & text [idx + mark . len () ..] ; if let Some (stripped_text) = text . strip_prefix ("!(") { text = stripped_text . trim_start () ; if let Some (idx2) = text . find (| c : char | ! (c . is_alphanumeric () || c == '_')) { let mark_text = & text [.. idx2] ; set . insert (mark_text . to_owned ()) ; text = & text [idx2 ..] ; } } } } }
};
}
