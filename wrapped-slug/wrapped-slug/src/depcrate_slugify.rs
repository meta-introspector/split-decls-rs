// Generated macro for _slugify (function)
macro_rules! Depcrate_slugify {
() => {
// Module: crate
// Provides: {"_slugify"}
// Dependencies: {}
fn _slugify (s : & str) -> String { let mut slug = String :: with_capacity (s . len ()) ; let mut prev_is_dash = true ; { let mut push_char = | x : u8 | { match x { b'a' ..= b'z' | b'0' ..= b'9' => { prev_is_dash = false ; slug . push (x . into ()) ; } b'A' ..= b'Z' => { prev_is_dash = false ; slug . push ((x - b'A' + b'a') . into ()) ; } _ => { if ! prev_is_dash { slug . push ('-') ; prev_is_dash = true ; } } } } ; for c in s . chars () { if c . is_ascii () { (push_char) (c as u8) ; } else { for & cx in deunicode_char (c) . unwrap_or ("-") . as_bytes () { (push_char) (cx) ; } } } } if slug . ends_with ('-') { slug . pop () ; } slug . shrink_to_fit () ; slug }
};
}
