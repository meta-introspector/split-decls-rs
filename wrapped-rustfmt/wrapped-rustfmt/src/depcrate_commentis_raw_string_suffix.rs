// Generated macro for is_raw_string_suffix (function)
macro_rules! Depcrate_commentis_raw_string_suffix {
() => {
// Module: crate::comment
// Provides: {"is_raw_string_suffix"}
// Dependencies: {}
fn is_raw_string_suffix < T > (iter : & mut MultiPeek < T > , count : u32) -> bool where T : Iterator , T :: Item : RichChar , { for _ in 0 .. count { match iter . peek () { Some (c) if c . get_char () == '#' => continue , _ => return false , } } true }
};
}
