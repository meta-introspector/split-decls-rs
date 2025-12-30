// Generated macro for contains_alphabetic_chars (function)
macro_rules! Depcrate_units_helperscontains_alphabetic_chars {
() => {
// Module: crate::units::helpers
// Provides: {"contains_alphabetic_chars"}
// Dependencies: {}
# [doc = " Determines if a string contains any alphabetic characters."] # [doc = " Returns true if the string contains at least one alphabetic character, false otherwise."] # [doc = " Examples:"] # [doc = " - \"1\" returns false"] # [doc = " - \"ft_to_m\" returns true"] # [doc = " - \"1E2\" returns true"] # [doc = " - \"1.5E-2\" returns true"] pub (crate) fn contains_alphabetic_chars (s : & str) -> bool { s . chars () . any (char :: is_alphabetic) }
};
}
