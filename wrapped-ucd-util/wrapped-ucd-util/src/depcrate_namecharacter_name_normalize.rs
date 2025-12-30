// Generated macro for character_name_normalize (function)
macro_rules! Depcrate_namecharacter_name_normalize {
() => {
// Module: crate::name
// Provides: {"character_name_normalize"}
// Dependencies: {}
# [doc = " Normalize the given character name in place according to UAX44-LM2."] # [doc = ""] # [doc = " See: https://unicode.org/reports/tr44/#UAX44-LM2"] pub fn character_name_normalize (string : & mut String) { let bytes = unsafe { string . as_mut_vec () } ; let len = character_name_normalize_bytes (bytes) . len () ; bytes . truncate (len) ; }
};
}
