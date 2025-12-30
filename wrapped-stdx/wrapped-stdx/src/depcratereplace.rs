// Generated macro for replace (function)
macro_rules! Depcratereplace {
() => {
// Module: crate
// Provides: {"replace"}
// Dependencies: {}
pub fn replace (buf : & mut String , from : char , to : & str) { let replace_count = buf . chars () . filter (| & ch | ch == from) . count () ; if replace_count == 0 { return ; } let from_len = from . len_utf8 () ; let additional = to . len () . saturating_sub (from_len) ; buf . reserve (additional * replace_count) ; let mut end = buf . len () ; while let Some (i) = buf [.. end] . rfind (from) { buf . replace_range (i .. i + from_len , to) ; end = i ; } }
};
}
