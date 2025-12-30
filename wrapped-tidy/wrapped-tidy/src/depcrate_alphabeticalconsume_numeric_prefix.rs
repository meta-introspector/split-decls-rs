// Generated macro for consume_numeric_prefix (function)
macro_rules! Depcrate_alphabeticalconsume_numeric_prefix {
() => {
// Module: crate::alphabetical
// Provides: {"consume_numeric_prefix"}
// Dependencies: {}
fn consume_numeric_prefix < I : Iterator < Item = char > > (it : & mut Peekable < I >) -> String { let mut result = String :: new () ; while let Some (& c) = it . peek () { if ! c . is_numeric () { break ; } result . push (c) ; it . next () ; } result }
};
}
