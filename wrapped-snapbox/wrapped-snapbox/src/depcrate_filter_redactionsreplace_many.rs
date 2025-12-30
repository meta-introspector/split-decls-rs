// Generated macro for replace_many (function)
macro_rules! Depcrate_filter_redactionsreplace_many {
() => {
// Module: crate::filter::redactions
// Provides: {"replace_many"}
// Dependencies: {}
# [doc = " Replacements is `(from, to)`"] fn replace_many < 'a > (buffer : & mut String , replacements : impl IntoIterator < Item = (& 'a RedactedValueInner , & 'a str) > ,) { for (var , replace) in replacements { let mut index = 0 ; while let Some (offset) = var . find_in (& buffer [index ..]) { let old_range = (index + offset . start) .. (index + offset . end) ; buffer . replace_range (old_range , replace) ; index += offset . start + replace . len () ; } } }
};
}
