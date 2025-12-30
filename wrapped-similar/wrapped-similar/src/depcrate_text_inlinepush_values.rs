// Generated macro for push_values (function)
macro_rules! Depcrate_text_inlinepush_values {
() => {
// Module: crate::text::inline
// Provides: {"push_values"}
// Dependencies: {}
fn push_values < 's , T : DiffableStr + ? Sized > (v : & mut Vec < Vec < (bool , & 's T) > > , idx : usize , emphasized : bool , s : & 's T ,) { v . resize_with (v . len () . max (idx + 1) , Vec :: new) ; if emphasized { for seg in s . tokenize_lines_and_newlines () { v [idx] . push ((! seg . ends_with_newline () , seg)) ; } } else { v [idx] . push ((false , s)) ; } }
};
}
